use super::{Node, NodeEditor};
use crate::system::Value;
use imnodes::{InputPinId, OutputPinId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Literal(crate::system::Value),
    Container(Vec<AttributeValue>),
    Dictionary(HashMap<String, AttributeValue>),
    Empty,
    Error(String),
}

impl AttributeValue {
    pub fn slider(name: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        if let AttributeValue::Literal(v) = value {
            match v {
                Value::FloatRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(name, min.clone(), max.clone()).build(ui, v);
                }
                Value::IntRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(name, min.clone(), max.clone()).build(ui, v);
                }
                _ => {}
            }
        }
    }

    pub fn input(label: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        if let AttributeValue::Literal(v) = value {
            match v {
                Value::TextBuffer(text) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputText::new(ui, label, text).build();
                }
                Value::Int(int) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputInt::new(ui, label, int).build();
                }
                Value::Float(float) => {
                    ui.set_next_item_width(130.0);
                    imgui::InputFloat::new(ui, label, float).build();
                }
                _ => {}
            }
        } else if let AttributeValue::Dictionary(map) = value {
            let selected = map.iter().find(|p| {
                if let (_, AttributeValue::Literal(Value::Bool(selected))) = p {
                    *selected
                } else {
                    false
                }
            });

            let preview_value = if let Some(s) = selected { s.0 } else { "" };

            ui.set_next_item_width(130.0);
            if let Some(t) = imgui::ComboBox::new(label)
                .preview_value(preview_value)
                .begin(ui)
            {
                for (attr_name, attr) in map {
                    if let AttributeValue::Literal(Value::Bool(selected)) = attr {
                        if imgui::Selectable::new(attr_name)
                            .selected(*selected)
                            .build(ui)
                        {
                            ui.set_item_default_focus();
                            ui.text(attr_name);
                            *selected = true;
                        } else {
                            *selected = false;
                        }
                    }
                }
                t.end();
            }
        }
    }
}

#[derive(Clone)]
pub enum NodeResource {
    Title(&'static str),
    Input(fn() -> &'static str, Option<imnodes::InputPinId>),
    Output(
        fn() -> &'static str,
        fn(state: &HashMap<String, AttributeValue>) -> Option<AttributeValue>,
        Option<AttributeValue>,
        Option<imnodes::OutputPinId>,
    ),
    Attribute(
        fn() -> &'static str,
        fn(name: String, ui: &imgui::Ui, attribute_value: &mut AttributeValue),
        Option<AttributeValue>,
        Option<imnodes::AttributeId>,
    ),
    OutputWithAttribute(
        fn() -> &'static str,
        fn(name: String, ui: &imgui::Ui, attribute_value: &mut AttributeValue),
        fn(state: &HashMap<String, AttributeValue>) -> Option<AttributeValue>,
        Option<AttributeValue>,
        Option<imnodes::OutputPinId>,
        Option<imnodes::AttributeId>,
    ),
}

impl NodeResource {
    pub fn index_editor_state(
        resources: Vec<EditorResource>,
    ) -> HashMap<imnodes::NodeId, AttributeValue> {
        // First get all of the links
        let links = resources.iter().filter_map(|r| {
            if let EditorResource::Link { start, end, .. } = r {
                Some((start, end))
            } else {
                None
            }
        });

        let mut inputid_to_nodeid_index: HashMap<InputPinId, imnodes::NodeId> = HashMap::new();
        let mut outputid_to_nodeid_index: HashMap<OutputPinId, imnodes::NodeId> = HashMap::new();
        let mut nodeid_to_dictionary: HashMap<imnodes::NodeId, AttributeValue> = HashMap::new();
        for editor_resource in resources.iter() {
            let index = NodeResource::index_node_inputs(editor_resource);
            inputid_to_nodeid_index.extend(index);

            let output_index = NodeResource::index_node_outputs(editor_resource);
            outputid_to_nodeid_index.extend(output_index);

            let dictionary = NodeResource::index_node_state_to_dictionary(editor_resource);
            nodeid_to_dictionary.extend(dictionary);
        }

        // Merge the attributes from
        for ((output_name, output_pin_id), (input_name, input_pin_id)) in links {
            // This looks like a lot but it's pretty straight-forward
            match match match outputid_to_nodeid_index.get(output_pin_id) {
                // First check to see if we have the output node
                Some(output_node_id) => match &nodeid_to_dictionary.get(output_node_id) {
                    Some(AttributeValue::Dictionary(output_values)) => Some(output_values),
                    _ => None,
                },
                None => None,
            } {
                // Next check to see if that output node has a value
                Some(output_values) => output_values.get(output_name),
                None => None,
            } {
                Some(output_val) => {
                    // Then update the state of the input node and add that value to it's state at the entry of the connected input 
                    match inputid_to_nodeid_index.get(input_pin_id) {
                        Some(input_node_id) => {
                            match &nodeid_to_dictionary.get(input_node_id) {
                                Some(AttributeValue::Dictionary(input_values)) => {
                                    let mut updated_input_values = input_values.clone();
                                    updated_input_values.insert(input_name.to_string(), output_val.clone());

                                    nodeid_to_dictionary.insert(
                                        *input_node_id,
                                        AttributeValue::Dictionary(updated_input_values),
                                    );
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            // TODO: This happens new connections, but stale connections need to be updated as well
            }
        }

        nodeid_to_dictionary
    }

    fn index_node_inputs(editor_resource: &EditorResource) -> HashMap<InputPinId, imnodes::NodeId> {
        let mut index: HashMap<InputPinId, imnodes::NodeId> = HashMap::new();

        if let EditorResource::Node {
            id: Some(id),
            resources,
        } = editor_resource
        {
            for r in resources.iter().filter_map(|f| match f {
                NodeResource::Input(_, Some(input_id)) => Some(input_id),
                _ => None,
            }) {
                index.insert(*r, *id);
            }
        }
        index
    }

    fn index_node_outputs(
        editor_resource: &EditorResource,
    ) -> HashMap<OutputPinId, imnodes::NodeId> {
        let mut index: HashMap<OutputPinId, imnodes::NodeId> = HashMap::new();

        if let EditorResource::Node {
            id: Some(id),
            resources,
        } = editor_resource
        {
            for r in resources.iter().filter_map(|f| match f {
                NodeResource::Output(_, _, Some(..), Some(output_id)) => Some(output_id),
                NodeResource::OutputWithAttribute(_, _, _, Some(..), Some(output_id), ..) => {
                    Some(output_id)
                }
                _ => None,
            }) {
                index.insert(*r, *id);
            }
        }
        index
    }

    // Indexes all of the attribute and output values to an AttributeVale::Dictionary
    // Returns a hashmap so that it can be merged with other maps
    fn index_node_state_to_dictionary(
        editor_resource: &EditorResource,
    ) -> HashMap<imnodes::NodeId, AttributeValue> {
        let mut index: HashMap<imnodes::NodeId, AttributeValue> = HashMap::new();
        let mut attribute_dictionary: HashMap<String, AttributeValue> = HashMap::new();

        if let EditorResource::Node {
            id: Some(id),
            resources,
        } = editor_resource
        {
            resources.iter().for_each(|f| match f {
                NodeResource::Attribute(name, _, Some(v), _) => {
                    attribute_dictionary.insert(name().to_string(), v.clone());
                }
                NodeResource::Output(name, _, Some(v), _) => {
                    attribute_dictionary.insert(name().to_string(), v.clone());
                }
                NodeResource::OutputWithAttribute(name, _, _, Some(v), _, _) => {
                    attribute_dictionary.insert(name().to_string(), v.clone());
                }
                NodeResource::Input(name, Some(..)) => {
                    attribute_dictionary.insert(name().to_string(), AttributeValue::Empty);
                }
                _ => {}
            });

            index.insert(*id, AttributeValue::Dictionary(attribute_dictionary));
        }

        index
    }
}

impl NodeResource {
    pub fn name(&self) -> String {
        match self {
            NodeResource::Title(s) => s.to_string(),
            NodeResource::Input(s, _) => s().to_string(),
            NodeResource::Output(s, ..) => s().to_string(),
            NodeResource::Attribute(s, _, _, _) => s().to_string(),
            NodeResource::OutputWithAttribute(s, _, _, _, _, _) => s().to_string(),
        }
    }

    pub fn debug_state(&self) -> String {
        match self {
            NodeResource::Title(_) => String::new(),
            NodeResource::Input(_, v) => format!("{:#?}", v),
            NodeResource::Output(_, _, o, v) => format!("{:#?} {:#?}", o, v),
            NodeResource::Attribute(_, _, v, _) => format!("{:#?}", v),
            NodeResource::OutputWithAttribute(_, _, _, v, o, a) => {
                format!("{:#?} {:#?} {:#?}", v, o, a)
            }
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
            NodeResource::OutputWithAttribute(
                name,
                display,
                _,
                attr,
                Some(output_id),
                Some(attr_id),
            ) => {
                let name = name();

                if let Some(attr) = attr {
                    node.attribute(attr_id.clone(), || {
                        display(name.to_string(), &ui, attr);
                    });
                }
                node.add_output(output_id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
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
                NodeResource::OutputWithAttribute(name, display, vfn, val, None, None) => {
                    NodeResource::OutputWithAttribute(
                        name,
                        display,
                        vfn,
                        val,
                        Some(id_gen.next_output_pin()),
                        Some(id_gen.next_attribute()),
                    )
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
        start: (String, imnodes::OutputPinId),
        end: (String, imnodes::InputPinId),
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
