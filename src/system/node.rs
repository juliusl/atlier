use std::collections::HashMap;

use imnodes::{editor, CoordinateSystem};
use imnodes::{EditorContext, IdentifierGenerator};

mod resource;
pub use resource::AttributeValue;
pub use resource::EditorResource;
pub use resource::NodeResource;

mod expression;
pub use expression::Sum;

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
                    editor.add_link(*id, *end, *start)
                }
            });

        if let (true, Some((debug, debug_attr))) = self.debug {
            editor.add_node(debug, |mut nodescope| {
                nodescope.add_titlebar(|| ui.text("debug"));
                nodescope.attribute(debug_attr, || {
                    ui.set_next_item_width(400.0);
                    if let Some(listboxtoken) = imgui::ListBox::new("debug state").size([400.0, 500.0]).begin(ui) {
                        if let Some(resources_tree) = imgui::TreeNode::new("resources").push(ui) {
                            self.resources.iter().for_each(|f| {
                                if let EditorResource::Node {
                                    id: Some(i),
                                    resources,
                                } = f
                                {
                                    let tree_label = format!("Node: {:?}", i);
        
                                    ui.set_next_item_width(120.0);
                                    if let Some(node_token) = imgui::TreeNode::new(tree_label).push(ui) {
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
                                    if let Some(link_item) = imgui::TreeNode::new(tree_label).push(ui) {
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
            self.debug = (true, Some((self.id_gen.next_node(), self.id_gen.next_attribute())));
        }

        let next_resources = self.resources.iter_mut().map(|r| {
            if let EditorResource::Node {
                id, 
                resources,
            } = r { 
                let selfres = resources.to_vec();
                if selfres.iter().any(|f| match f { NodeResource::Output(..) => true, _ => false }) {
    
                    let update: Vec<NodeResource> = selfres.iter().map(|n| {

                    let mut state = HashMap::<String, Vec<NodeResource>>::new();
                    state.insert("self".to_string(), selfres.to_vec());
                        if let NodeResource::Output(v, func, _, i) = n { 
                            let next = NodeResource::Output(v.to_owned(), func.to_owned(), func(state), *i);
                            return next;
                        } else {
                            return n.to_owned();
                        }
                    }).collect();
    
                    return EditorResource::Node {
                        resources: update,
                        id: id.clone(),
                    };
                }
                return r.to_owned();
            } else {
                return r.to_owned();
            }
        });

        self.resources = next_resources.collect();
    }

    fn get_state(&self) -> Vec<EditorResource> {
        self.resources.to_vec()
    }
}

impl<'a> NodeEventHandler for NodeModule {
    fn on_node_link_created(&mut self, link: imnodes::Link) {
        if let (false, s, e) = (link.craeated_from_snap, link.start_pin, link.end_pin) {
            let exists = self.resources.iter().any(|f| {
                if let EditorResource::Link { end, start, .. } = f {
                    return e == *end && s == *start;
                }

                return false;
            });

            if !exists {
                let linkid = self.id_gen.next_link();

                self.resources.push(EditorResource::Link {
                    id: linkid,
                    start: s,
                    end: e,
                });
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
