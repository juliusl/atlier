

use imnodes::editor;
use imnodes::{EditorContext, IdentifierGenerator};


mod resource;
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
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource>;

    fn show(&mut self, editor: &mut imnodes::EditorScope, ui: &imgui::Ui);
}

pub trait Node {
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        resources: Vec<NodeResource>,
    ) -> Vec<NodeResource>;

    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui);
}

pub struct NodeModule {
    resources: Vec<EditorResource>,
    id_gen: IdentifierGenerator,
    links: Vec<(imnodes::LinkId, imnodes::InputPinId, imnodes::OutputPinId)>,
}

impl<'a> NodeEditor for NodeModule {
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_context: &imnodes::EditorContext,
        resources: Vec<EditorResource>,
    ) -> Vec<EditorResource> {
        EditorResource::setup(id_gen, editor_context, resources.to_owned())
    }

    fn show(&mut self, mut editor: &mut imnodes::EditorScope, ui: &imgui::Ui) {
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
            
        self.links.iter().for_each(|l| {
            editor.add_link(l.0, l.1, l.2);
        })
    }
}

pub struct NodeApp {
    name: String,
    modules: Vec<(EditorContext, NodeModule)>,
    imnode: imnodes::Context,
}

impl NodeApp {
    pub fn new(name: String) -> Self {
        NodeApp {
            name: name,
            imnode: imnodes::Context::new(),
            modules: vec![],
        }
    }

    pub fn with(mut self, resources: Vec<EditorResource>)  -> Self {
        let editor_context = self.imnode.create_editor();
        let id_gen = editor_context.new_identifier_generator();
        self.modules.push((
            editor_context,
            NodeModule {
                resources: resources,
                id_gen: id_gen,
                links: vec![],
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

impl<'a> NodeEventHandler for NodeModule {
    fn on_node_link_created(&mut self, link: imnodes::Link) {
        if let (false, start, end) = (link.craeated_from_snap, link.start_pin, link.end_pin) {
            let exists = self.links.iter().any(|f| f.1 == end && f.2 == start);
            if !exists {
                let linkid = self.id_gen.next_link();

                self.links.push((linkid, end, start));
            }
        }
    }

    fn on_node_link_destroyed(&mut self, linkid: imnodes::LinkId) {
        let next: Vec<(imnodes::LinkId, imnodes::InputPinId, imnodes::OutputPinId)> = self
            .links
            .iter_mut()
            .filter(|(l, ..)| *l != linkid)
            .map(|f| *f)
            .collect();

        self.links = next;
    }
}
