use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use imnodes::{editor, CoordinateSystem};
use imnodes::{EditorContext, IdentifierGenerator};

mod resource;
pub use resource::AttributeValue;
pub use resource::EditorResource;
pub use resource::NodeResource;

pub mod expression;
pub use expression::ExpressionVisitor;

mod visitor;

use super::App;

pub trait NodeEventHandler {
    fn on_node_link_created(&mut self, link: imnodes::Link);
    fn on_node_link_destroyed(&mut self, linid: imnodes::LinkId);
}

pub trait NodeEditor {
    type State;

    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource>;

    fn show(&mut self, editor: &mut imnodes::EditorScope, ui: &imgui::Ui);

    // Return the current state
    fn get_state(&self) -> Vec<Self::State>;

    fn context_menu(&mut self, ui: &imgui::Ui);
}

pub trait Node {
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        resources: Vec<NodeResource>,
    ) -> Vec<NodeResource>;

    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui);
}


// NodeModule encapsulates a single editor and it's resources
// it is an node event handler and will detect new connections
pub struct NodeModule {
    resources: Vec<EditorResource>,
    id_gen: IdentifierGenerator,
    debug: (bool, Option<(imnodes::NodeId, imnodes::AttributeId)>),
    state: (u64, Option<HashMap<imnodes::NodeId, AttributeValue>>),
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

impl<'a> NodeEditor for NodeModule {
    type State = EditorResource;

    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource> {
        EditorResource::setup(id_gen, editor_context, resources.to_owned())
    }

    fn show(&mut self, mut editor: &mut imnodes::EditorScope, ui: &imgui::Ui) {
        self.context_menu(ui);

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
                let editor_scope = &mut editor;

                res.show(editor_scope, ui)
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
                                                    if let AttributeValue::Map(map) = v {
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
                                                    "{:?}\n{:?}\n{:?}",
                                                    i.get_position(CoordinateSystem::ScreenSpace),
                                                    i.get_position(CoordinateSystem::EditorSpace), // TODO: These two appear to be the same
                                                    i.get_position(CoordinateSystem::GridSpace),
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
                let self_res = resources.to_vec();
                if self_res.iter().any(|f| match f {
                    NodeResource::Output(..) => true,
                    _ => false,
                }) {
                    let update: Vec<NodeResource> = self_res
                        .iter()
                        .map(|n| {
                            if let Some(state) = &self.state.1 {
                                if let Some(AttributeValue::Map(state)) = state.get(nodeid) {
                                    match n {
                                        NodeResource::Output(v, func, _, i) => {
                                            let next = NodeResource::Output(
                                                v.to_owned(),
                                                func.to_owned(),
                                                func(state),
                                                *i,
                                            );
                                            return next;
                                        }
                                        NodeResource::OutputWithAttribute(
                                            v,
                                            display,
                                            output,
                                            _,
                                            output_id,
                                            attr_id,
                                        ) => {
                                            let next = NodeResource::OutputWithAttribute(
                                                v.to_owned(),
                                                display.to_owned(),
                                                output.to_owned(),
                                                output(state),
                                                *output_id,
                                                *attr_id,
                                            );
                                            return next;
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
                }
                return r.to_owned();
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
                self.resources.clone().iter().filter(|r| match r {
                    EditorResource::Node { .. } => true,
                    _ => false,
                }).for_each(|editor_resource| {
                    // TODO: This can be optimized by building a node cache on initialization, that way all nodes aren't iterated over and over
                    let title = editor_resource
                        .get_state()
                        .clone()
                        .iter()
                        .find_map(|f| match f {
                            NodeResource::Title(t) => Some(t.clone()),
                            _ => None,
                        });

                    match title {
                        Some(title) if added.insert(title.to_string()) => if imgui::MenuItem::new(title).build(ui) {
                                    match editor_resource {
                                        EditorResource::Node { .. } => {
                                            let new_node = self.id_gen.next_node();
                                            self.resources
                                                .push(editor_resource.copy_blank(Some(new_node)));

                                            new_node.set_position(
                                                pos[0],
                                                pos[1],
                                                CoordinateSystem::ScreenSpace,
                                            );
                                        }
                                        _ => (),
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
        if let (false, s, e, start_node, end_node) = (
            link.craeated_from_snap,
            link.start_pin,
            link.end_pin,
            link.start_node,
            link.end_node,
        ) {
            let exists = self.resources.iter().any(|f| {
                if let EditorResource::Link { end, start, .. } = f {
                    return e == end.1 && s == start.1;
                }

                return false;
            });

            if !exists {
                let linkid = self.id_gen.next_link();

                let output_name = self.resources.iter().find_map(|f| {
                    if let EditorResource::Node {
                        id: Some(id),
                        resources,
                    } = f
                    {
                        if *id == start_node {
                            let name = resources.iter().find_map(|f| match f {
                                NodeResource::Output(start_node_name, _, _, Some(start_id)) => {
                                    if *start_id == s {
                                        Some(start_node_name())
                                    } else {
                                        None
                                    }
                                }
                                NodeResource::OutputWithAttribute(
                                    start_node_name,
                                    _,
                                    _,
                                    _,
                                    Some(start_id),
                                    _,
                                ) => {
                                    if *start_id == s {
                                        Some(start_node_name())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            });
                            return name;
                        }
                        None
                    } else {
                        None
                    }
                });

                let input_name = self.resources.iter().find_map(|f| {
                    if let EditorResource::Node {
                        id: Some(id),
                        resources,
                    } = f
                    {
                        if *id == end_node {
                            let name = resources.iter().find_map(|f| {
                                if let NodeResource::Input(end_node_name, Some(end_id)) = f {
                                    if *end_id == e {
                                        Some(end_node_name())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            });
                            return name;
                        }
                        None
                    } else {
                        None
                    }
                });

                if let (Some(output_name), Some(input_name)) = (output_name, input_name) {
                    self.resources.push(EditorResource::Link {
                        id: linkid,
                        start: (output_name.to_string(), s),
                        end: (input_name.to_string(), e),
                    });
                }
            }
        }
    }

    fn on_node_link_destroyed(&mut self, linkid: imnodes::LinkId) {
        let next: Vec<EditorResource> = self
            .resources
            .iter()
            .filter(|l| {
                if let EditorResource::Link { id, .. } = l {
                    return *id != linkid;
                } else {
                    return true;
                }
            })
            .map(|f| f.to_owned())
            .collect();

        self.resources = next;
    }
}

pub struct NodeApp {
    name: String,
    modules: Vec<(EditorContext, NodeModule)>,
    imnode: imnodes::Context,
}

impl NodeApp {
    // Instantiates a new node editor window
    pub fn new(name: String) -> Self {
        NodeApp {
            name: name,
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

impl<'a> App<'a> for NodeApp {
    fn get_window(&self) -> imgui::Window<'static, String> {
        imgui::Window::new(self.name.clone())
            .resizable(true)
            .movable(true)
            .position([0.0, 0.0], imgui::Condition::Once)
            .size([800.0, 600.0], imgui::Condition::Once)
    }

    fn show(&mut self, ui: &imgui::Ui) {
        let window = self.get_window();

        window.build(&ui, || {
            self.modules.iter_mut().for_each(|(e, m)| {
                let resources =
                    <NodeModule as NodeEditor>::setup(&mut m.id_gen, &e, m.resources.to_vec());
                m.resources = resources;

                let detatch = e.push(imnodes::AttributeFlag::EnableLinkDetachWithDragClick);

                let outer_scope = editor(e, |mut editor| m.show(&mut editor, ui));

                for i in outer_scope.links_created() {
                    m.on_node_link_created(i);
                }

                for i in outer_scope.get_dropped_link() {
                    m.on_node_link_destroyed(i);
                }

                detatch.pop();
            });
        });
    }
}
