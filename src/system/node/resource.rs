use imnodes::AttributeFlag;

use super::{Node, NodeEditor};

#[derive(Clone, Copy)]
pub enum NodeResource {
    Title(&'static str), 
    Input(fn() -> &'static str, Option<imnodes::InputPinId>), 
    Output(fn() -> &'static str, Option<imnodes::OutputPinId>), 
    Attribute(fn() -> &'static str, fn(name: String, ui: &imgui::Ui), Option<imnodes::AttributeId>),
}

impl Node for NodeResource {
    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui) {
        match self {
            NodeResource::Title(title) => {
                node.add_titlebar(||{
                    ui.text(title)
                })
            }
            NodeResource::Input(name, Some(id))  => {
                let name = name();
                node.add_input(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Output(name, Some(id)) => {
                let name = name();
                node.add_output(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Attribute(name, display, Some(id)) => {
                let name = name();
                node.attribute(id.clone(), ||{
                    display(name.to_string(), &ui);
                });
            }
            _ => return
        }
    }

    fn setup(id_gen: &mut imnodes::IdentifierGenerator, resources: Vec<NodeResource>) -> Vec<NodeResource> {
        let mut next = vec![];
        for r in resources {
           let next_r = match r {
                NodeResource::Attribute(name, display, None) => {
                    NodeResource::Attribute(name, display, Some(id_gen.next_attribute()))
                }
                NodeResource::Output(name, None) => {
                    NodeResource::Output(name, Some(id_gen.next_output_pin()))
                }
                NodeResource::Input(name, None) => {
                    NodeResource::Input(name, Some(id_gen.next_input_pin()))
                }
                NodeResource::Title(title) => NodeResource::Title(title),
                p => p.clone(),
            };

            next.push(next_r); 
        }
        
        next
    }
}

#[derive(Clone)]
pub enum EditorResource {    
    Node {
        id: Option<imnodes::NodeId>,
        resources: Vec<NodeResource>,
    },
    ColorStyle(fn(editor_context: &imnodes::EditorContext) -> imnodes::ColorToken, Option<imnodes::ColorToken>),
    AttributeFlag(fn(editor_context: &imnodes::EditorContext) -> imnodes::AttributeFlagToken, Option<imnodes::AttributeFlagToken>),
}

impl EditorResource {
    pub fn create_link_on_snap() -> EditorResource {
        EditorResource::AttributeFlag(|e|e.push(AttributeFlag::EnableLinkCreationOnSnap), None)
    }
    
    pub fn detatch_with_drag_click() -> EditorResource {
        EditorResource::AttributeFlag(|e|e.push(AttributeFlag::EnableLinkDetachWithDragClick), None)
    }
}

impl NodeEditor for EditorResource {
    fn setup(id_gen: &mut imnodes::IdentifierGenerator, _: &imnodes::EditorContext, mut resources: Vec<EditorResource>)  -> Vec<EditorResource> {
        let mut next = vec![];
        
        for r in resources.iter_mut() {
            let next_r = match r {
                EditorResource::Node {
                    id: None,
                    resources,
                } => {
                    EditorResource::Node {
                        id: Some(id_gen.next_node()),
                        resources:  NodeResource::setup(id_gen, resources.to_vec()),
                    }
                },
                p => p.clone(),
            };
        
            next.push(next_r);
        }

        next
    }

    fn show(&mut self, editor: &mut imnodes::EditorScope, ui: &imgui::Ui) {       
        match self {
            EditorResource::Node {
                id: Some(id),
                resources,
            } => {
                editor.add_node(id.clone(), |mut scope|{     
                    let mut iter = resources.iter_mut();

                    while let Some(next) = iter.next() {
                        let node_scope = &mut scope;

                        next.show(node_scope, ui)
                    }
                })
            },
            _ => {}
        };
    }
}
