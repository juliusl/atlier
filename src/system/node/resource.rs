use super::{Node, NodeEditor};
use crate::system::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AttributeValue {
    System(crate::system::Value),
}

impl AttributeValue {
    pub fn slider(name: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        match value {
            AttributeValue::System(v) => match v {
                Value::FloatRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(name, min.clone(), max.clone()).build(ui, v);
                }
                Value::IntRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(name, min.clone(), max.clone()).build(ui, v);
                },
                _ => {} 
            }
        };
    }

    pub fn input(label: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        match value {
            AttributeValue::System(v) => match v {
                Value::TextBuffer(text) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputText::new(ui, label, text).build();
                },
                Value::Int(int) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputInt::new(ui, label, int).build();
                },
                Value::Float(float) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputFloat::new(ui, label, float).build();
                },
                _ => {},
            }
        }
    }
}

#[derive(Clone)]
pub enum NodeResource {
    Title(&'static str),
    Input(fn() -> &'static str, Option<imnodes::InputPinId>), //
    Output(fn() -> &'static str, 
        fn(state: HashMap::<String, Vec<NodeResource>>) -> Option<AttributeValue>, 
        Option<AttributeValue>, 
        Option<imnodes::OutputPinId>), // <- this is the visitor
    Attribute(
        fn() -> &'static str,
        fn(name: String, ui: &imgui::Ui, attribute_value: &mut AttributeValue),
        Option<AttributeValue>,
        Option<imnodes::AttributeId>,
    ),
}

impl NodeResource {
    pub fn name(&self) -> String {
        match self {
            NodeResource::Title(s) => s.to_string(),
            NodeResource::Input(s, _) => s().to_string(),
            NodeResource::Output(s,..) => s().to_string(),
            NodeResource::Attribute(s, _, _, _) => s().to_string(),
        }
    }

    pub fn debug_state(&self) -> String {
        match self {
            NodeResource::Title(_) => String::new(),
            NodeResource::Input(_, v) => format!("{:#?}", v),
            NodeResource::Output(_, _, o, v) => format!("{:#?} {:#?}",  o, v),
            NodeResource::Attribute(_, _, v, _) => format!("{:#?}", v),
        }
    }
}

impl Node for NodeResource {
    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui) {
        match self {
            NodeResource::Title(title) => node.add_titlebar(|| ui.text(title)),
            NodeResource::Input(name, Some(id)) => {
                let name = name();
                node.add_input(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Output(name, _, Some(_), Some(id)) => {
                let name = name();
                node.add_output(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Attribute(name, display, Some(attr), Some(id)) => {
                let name = name();
                node.attribute(id.clone(), || {
                    display(name.to_string(), &ui, attr);
                });
            }
            _ => return,
        }
    }

    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        resources: Vec<NodeResource>,
    ) -> Vec<NodeResource> {
        let mut next = vec![];
        for r in resources {
            let next_r = match r {
                NodeResource::Attribute(name, display, Some(v), None) => {
                    NodeResource::Attribute(name, display, Some(v), Some(id_gen.next_attribute()))
                }
                NodeResource::Output(name, vfn, v, None) => {
                    NodeResource::Output(name, vfn, v, Some(id_gen.next_output_pin()))
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
    Link {
        id: imnodes::LinkId,
        start: imnodes::OutputPinId, 
        end: imnodes::InputPinId
    },
}

impl NodeEditor for EditorResource {
    type State = NodeResource;
    fn setup(
        id_gen: &mut imnodes::IdentifierGenerator,
        _: &imnodes::EditorContext,
        mut resources: Vec<EditorResource>,
    ) -> Vec<EditorResource> {
        let mut next = vec![];

        for r in resources.iter_mut() {
            let next_r = match r {
                EditorResource::Node {
                    id: None,
                    resources,
                } => EditorResource::Node {
                    id: Some(id_gen.next_node()),
                    resources: NodeResource::setup(id_gen, resources.to_vec()),
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
            } => editor.add_node(id.clone(), |mut scope| {
                let mut iter = resources.iter_mut();

                while let Some(next) = iter.next() {
                    let node_scope = &mut scope;

                    next.show(node_scope, ui)
                }
            }),
            _ => {}
        };
    }

    fn get_state(&self) -> Vec<Self::State> {
        match self {
            EditorResource::Node { resources, .. } => resources.to_vec(),
            _ => vec![],
        }
    }
}
