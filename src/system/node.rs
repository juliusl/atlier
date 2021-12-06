use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use imnodes::{editor, CoordinateSystem, NodeId};
use imnodes::{EditorContext, IdentifierGenerator};

mod resource;
pub use resource::EditorResource;
pub use resource::NodeResource;

pub mod expression;
pub use expression::*;

mod module;
pub use module::Initializer;
pub use module::Module;

mod visitor;
pub use visitor::*;

use super::{App, Attribute, State};

pub trait NodeEventHandler {
    fn on_node_link_created(&mut self, link: imnodes::Link);
    fn on_node_link_destroyed(&mut self, linid: imnodes::LinkId);
}

pub trait EditorComponent {
    type State;

    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource>;

    fn show(&mut self, editor: &mut imnodes::EditorScope, ui: &imgui::Ui, state: Option<&State>);

    // Return the current state
    fn get_state(&self) -> Vec<Self::State>;

    fn context_menu(&mut self, ui: &imgui::Ui);
}

pub trait NodeComponent {
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        resources: Vec<NodeResource>,
    ) -> Vec<NodeResource>;

    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui, state: &State);
}

// NodeModule encapsulates a single editor and it's resources
// it is an node event handler and will detect new connections
pub struct NodeModule {
    resources: Vec<EditorResource>,
    id_gen: IdentifierGenerator,
    debug: (bool, Option<(imnodes::NodeId, imnodes::AttributeId)>),
    state: (u64, Option<HashMap<imnodes::NodeId, Attribute>>),
}

pub fn begin_context_menu<'a>(
    popup_id: impl AsRef<str>,
    ui: &'a imgui::Ui<'a>,
) -> Option<imgui::PopupToken<'a>> {
    if ui.is_mouse_released(imgui::MouseButton::Right) {
        ui.open_popup(&popup_id);
    }

    ui.begin_popup(popup_id)
}

impl<'a> NodeModule {
    fn get_node_resource_name_from_link(
        &mut self,
        nodeid: NodeId,
        link: imnodes::Link,
    ) -> Option<&'static str> {
        let imnodes::Link {
            start_pin, end_pin, ..
        } = link;
        self.resources.iter().find_map(|f| match f {
            EditorResource::Node {
                id: Some(id),
                resources,
            } if *id == nodeid => resources.iter().find_map(|f| match f {
                NodeResource::Output(start_node_name, _, _, Some(start_id))
                | NodeResource::Reducer(start_node_name, _, _, _, _, Some(start_id), _)
                    if *start_id == start_pin =>
                {
                    Some(start_node_name())
                }
                NodeResource::Input(end_node_name, Some(end_id)) if *end_id == end_pin => {
                    Some(end_node_name())
                }
                _ => None,
            }),
            _ => None,
        })
    }

    fn get_state_for_resource(
        states: Option<&HashMap<NodeId, Attribute>>,
        res: &EditorResource,
    ) -> Option<State> {
        if let Some(map) = states {
            if let EditorResource::Node {
                id: Some(nodeid), ..
            } = res
            {
                if let Some(Attribute::Map(m)) = map.get(&nodeid) {
                    return Some(State::from(m));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

impl<'a> EditorComponent for NodeModule {
    type State = EditorResource;

    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource> {
        EditorResource::setup(id_gen, editor_context, resources.to_owned())
    }

    fn show(&mut self, mut editor: &mut imnodes::EditorScope, ui: &imgui::Ui, _: Option<&State>) {
        // Render nodes
        self.resources
            .iter_mut()
            .filter(|r| {
                if let EditorResource::Node { .. } = r {
                    return true;
                } else {
                    return false;
                }
            })
            .for_each(|res| {
                if let (_, Some(states)) = &self.state {
                    let borrowed: &EditorResource = res.borrow();
                    if let Some(s) = Self::get_state_for_resource(Some(&states), borrowed) {
                        let editor_scope = &mut editor;
                        res.show(editor_scope, ui, Some(&s));
                    }
                }
            });

        // Render links
        self.resources
            .iter()
            .filter(|r| {
                if let EditorResource::Link { .. } = r {
                    return true;
                } else {
                    return false;
                }
            })
            .for_each(|l| {
                if let EditorResource::Link { id, start, end } = l {
                    editor.add_link(*id, end.1, start.1)
                }
            });

        self.context_menu(ui);

        if let (true, Some((debug, debug_attr))) = self.debug {
            editor.add_node(debug, |mut nodescope| {
                nodescope.add_titlebar(|| ui.text("debug"));
                nodescope.attribute(debug_attr, || {
                    ui.set_next_item_width(400.0);
                    if let Some(listboxtoken) = imgui::ListBox::new("debug state")
                        .size([400.0, 500.0])
                        .begin(ui)
                    {
                        if let Some(resources_tree) = imgui::TreeNode::new("resources").push(ui) {
                            self.resources.iter().for_each(|f| {
                                if let EditorResource::Node {
                                    id: Some(i),
                                    resources,
                                } = f
                                {
                                    let tree_label = format!("Node: {:?}", i);
                                    ui.set_next_item_width(120.0);
                                    if let Some(node_token) =
                                        imgui::TreeNode::new(tree_label).push(ui)
                                    {
                                        if let (code, Some(state_index)) = &self.state {
                                            if let Some(v) = state_index.get(i) {
                                                let tree_label = format!("state_index ({})", code);

                                                if let Some(node_id_token) =
                                                    imgui::TreeNode::new(tree_label).push(ui)
                                                {
                                                    if let Attribute::Map(map) = v {
                                                        for (k, v) in map {
                                                            if let Some(dictionary_value_token) =
                                                                imgui::TreeNode::new(k).push(ui)
                                                            {
                                                                ui.text(format!("{:#?}", v));

                                                                dictionary_value_token.pop();
                                                            }
                                                        }
                                                    }
                                                    node_id_token.pop();
                                                }
                                            }
                                        }

                                        resources.iter().for_each(|n| {
                                            let name = &n.name();
                                            let state = &n.debug_state();
                                            ui.set_next_item_width(120.0);
                                            if let Some(node_resource_token) =
                                                imgui::TreeNode::new(name).push(ui)
                                            {
                                                ui.text(format!(
                                                    "{:?}\n{:?}\n{:?}\n{:?}",
                                                    i.get_position(CoordinateSystem::ScreenSpace),
                                                    i.get_position(CoordinateSystem::EditorSpace), // TODO: These two appear to be the same
                                                    i.get_position(CoordinateSystem::GridSpace),
                                                    i.get_dimensions(),
                                                ));
                                                ui.text(name);
                                                ui.text(state);

                                                node_resource_token.pop();
                                            }
                                        });

                                        node_token.pop();
                                    }
                                };

                                if let EditorResource::Link { start, end, id } = f {
                                    let tree_label = format!("Link: {:?}", *id);

                                    ui.set_next_item_width(120.0);
                                    if let Some(link_item) =
                                        imgui::TreeNode::new(tree_label).push(ui)
                                    {
                                        ui.text("linked--");

                                        let start = format!("{:#?}", start);
                                        let end = format!("{:#?}", end);

                                        ui.text(start);
                                        ui.text(end);
                                        link_item.pop();
                                    }
                                }
                            });
                            resources_tree.pop();
                        }

                        listboxtoken.end();
                    }
                });
            });
        } else if let (true, None) = self.debug {
            self.debug = (
                true,
                Some((self.id_gen.next_node(), self.id_gen.next_attribute())),
            );
        }

        let next_resources = self.resources.iter_mut().map(|r| {
            if let EditorResource::Node {
                id: Some(nodeid),
                resources,
            } = r
            {
                let update: Vec<NodeResource> = resources
                    .to_vec()
                    .iter()
                    .map(|n| {
                        if let (_, Some(state)) = &self.state {
                            if let Some(Attribute::Map(state)) = state.get(nodeid) {
                                match n {
                                    NodeResource::Output(v, func, _, i) => {
                                        let next = NodeResource::Output(
                                            v.to_owned(),
                                            func.to_owned(),
                                            func(State::from(state)),
                                            *i,
                                        );
                                        return next;
                                    }
                                    NodeResource::Reducer(
                                        name,
                                        display,
                                        map,
                                        reduce,
                                        (hash_code, reduced_state),
                                        output_id,
                                        attr_id,
                                    ) => {
                                        let (next_hash_code, next_param) = map(State::from(state));

                                        let next = if next_hash_code != *hash_code {
                                            let next_state = reduce(next_param);
                                            NodeResource::Reducer(
                                                name.to_owned(),
                                                display.to_owned(),
                                                map.to_owned(),
                                                reduce.to_owned(),
                                                (next_hash_code, next_state),
                                                *output_id,
                                                *attr_id,
                                            )
                                        } else {
                                            NodeResource::Reducer(
                                                name.to_owned(),
                                                display.to_owned(),
                                                map.to_owned(),
                                                reduce.to_owned(),
                                                (*hash_code, reduced_state.to_owned()),
                                                *output_id,
                                                *attr_id,
                                            )
                                        };
                                        next
                                    }
                                    _ => return n.to_owned(),
                                }
                            } else {
                                n.to_owned()
                            }
                        } else {
                            return n.to_owned();
                        }
                    })
                    .collect();

                return EditorResource::Node {
                    resources: update,
                    id: Some(*nodeid),
                };
            } else {
                return r.to_owned();
            }
        });

        self.resources = next_resources.collect();

        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        self.resources.hash(&mut hasher);
        let hash_code = hasher.finish();

        if let (_, None) = self.state {
            let state_index = NodeResource::index_editor_state(self.resources.to_vec());
            self.state = (hash_code, Some(state_index));
        } else if self.state.0 != hash_code {
            let state_index = NodeResource::index_editor_state(self.resources.to_vec());
            self.state = (hash_code, Some(state_index));
        }
    }

    fn get_state(&self) -> Vec<EditorResource> {
        self.resources.to_vec()
    }

    fn context_menu(&mut self, ui: &imgui::Ui) {
        let window_padding = ui.push_style_var(imgui::StyleVar::WindowPadding([16.0, 8.0]));
        if let Some(popup_token) = begin_context_menu("editor_context_menu", ui) {
            let pos = ui.mouse_pos_on_opening_current_popup();
            let mut added: HashSet<String> = HashSet::new();
            if let Some(expression_menu_token) = ui.begin_menu("Expressions") {
                self.resources
                    .clone()
                    .iter()
                    .filter(|r| match r {
                        EditorResource::Node { .. } => true,
                        _ => false,
                    })
                    .for_each(|editor_resource| {
                        // TODO: This can be optimized by building a node cache on initialization, that way all nodes aren't iterated over and over
                        let title =
                            editor_resource
                                .get_state()
                                .clone()
                                .iter()
                                .find_map(|f| match f {
                                    NodeResource::Title(t) => Some(t.clone()),
                                    _ => None,
                                });

                        match title {
                            Some(title) if added.insert(title.to_string()) => {
                                if imgui::MenuItem::new(title).build(ui) {
                                    match editor_resource {
                                        EditorResource::Node { .. } => {
                                            let new_node = self.id_gen.next_node();
                                            self.resources
                                                .push(editor_resource.copy_blank(Some(new_node)));

                                            // new_node.set_position(
                                            //     pos[0],
                                            //     pos[1],
                                            //     CoordinateSystem::ScreenSpace,
                                            // );
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        }
                    });
                expression_menu_token.end();
            }
            popup_token.end();
        }
        window_padding.pop();
    }
}

impl<'a> NodeEventHandler for NodeModule {
    fn on_node_link_created(&mut self, link: imnodes::Link) {
        let imnodes::Link {
            start_node,
            end_node,
            start_pin,
            end_pin,
            craeated_from_snap,
        } = link;

        let exists = self.resources.iter().any(|f| {
            match f {
                EditorResource::Link {
                            end: (_, end),
                            start: (_, start),
                            ..
                        } 
                if end_pin == *end && start_pin == *start => true,
                _ => false,
            }
        });

        if !exists && !craeated_from_snap {
            let linkid = self.id_gen.next_link();

            if let (Some(output_name), Some(input_name)) = (
                self.get_node_resource_name_from_link(start_node, link),
                self.get_node_resource_name_from_link(end_node, link),
            ) {
                self.resources.push(EditorResource::Link {
                    id: linkid,
                    start: (output_name.to_string(), start_pin),
                    end: (input_name.to_string(), end_pin),
                });
            }
        }
    }

    fn on_node_link_destroyed(&mut self, linkid: imnodes::LinkId) {
        let next: Vec<EditorResource> = self
            .resources
            .iter()
            .filter_map(|l| {
                if let EditorResource::Link { id, .. } = l {
                    if *id != linkid {
                        Some(l.to_owned())
                    } else {
                        None
                    }
                } else {
                    Some(l.to_owned())
                }
            })
            .collect();

        self.resources = next;
    }
}

pub struct NodeEditor {
    name: String,
    modules: Vec<(EditorContext, NodeModule)>,
    imnode: imnodes::Context,
}

impl NodeEditor {
    // Instantiates a new node editor window
    pub fn new(name: &str) -> Self {
        NodeEditor {
            name: name.to_string(),
            imnode: imnodes::Context::new(),
            modules: vec![],
        }
    }

    // Instantiates a new module to include with this app
    pub fn module(mut self, resources: Vec<EditorResource>, enable_debug: bool) -> Self {
        let editor_context = self.imnode.create_editor();
        let id_gen = editor_context.new_identifier_generator();
        self.modules.push((
            editor_context,
            NodeModule {
                resources: resources,
                id_gen: id_gen,
                debug: (enable_debug, None),
                state: (0, None),
            },
        ));

        self
    }
}

impl<'a> App<'a> for NodeEditor {
    fn get_window(&self) -> imgui::Window<'static, String> {
        imgui::Window::new(self.name.clone())
            .resizable(true)
            .movable(true)
            .position([0.0, 0.0], imgui::Condition::Once)
            .size([1920.0, 1080.0], imgui::Condition::Once)
    }

    fn show(&mut self, ui: &imgui::Ui) {
        let window = self.get_window();

        ui.show_demo_window(&mut true);

        window.build(&ui, || {
            imgui::ChildWindow::new("editor")
                .size([-1.0, 0.00])
                .build(ui, || {
                    self.modules.iter_mut().for_each(|(e, m)| {
                        e.set_style_colors_classic();

                        let node_padding =
                            imnodes::StyleVar::NodePaddingHorizontal.push_val(16.0, e);
                        let resources = <NodeModule as EditorComponent>::setup(
                            &mut m.id_gen,
                            &e,
                            m.resources.to_vec(),
                        );
                        m.resources = resources;

                        let detatch = e.push(imnodes::AttributeFlag::EnableLinkDetachWithDragClick);

                        let outer_scope = editor(e, |mut editor| m.show(&mut editor, ui, None));

                        for i in outer_scope.links_created() {
                            m.on_node_link_created(i);
                        }

                        for i in outer_scope.get_dropped_link() {
                            m.on_node_link_destroyed(i);
                        }

                        detatch.pop();
                        node_padding.pop();
                    });
                });
        });
    }
}
