use super::{Node, NodeEditor};
use crate::system::Value;
use imnodes::{InputPinId, OutputPinId};
use std::{
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Hash)]
pub enum AttributeValue {
    Literal(crate::system::Value),
    Map(BTreeMap<String, AttributeValue>),
    Empty,
    Error(String),
}

impl Into<f32> for AttributeValue {
    fn into(self) -> f32 {
        match self {
            AttributeValue::Literal(l) => match l {
                Value::Float(f) => f,
                Value::Int(i) => i as f32,
                Value::FloatRange(f, _, _) => f,
                Value::IntRange(i, _, _) => i as f32,
                _ => 0.00,
            },
            _ => 0.00,
        }
    }
}

impl Into<i32> for AttributeValue {
    fn into(self) -> i32 {
        match self {
            AttributeValue::Literal(l) => match l {
                Value::Float(f) => f as i32,
                Value::Int(i) => i,
                Value::FloatRange(f, _, _) => f as i32,
                Value::IntRange(i, _, _) => i,
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl AttributeValue {
    pub fn copy_blank(&self) -> Self {
        match self {
            AttributeValue::Literal(l) => match l {
                Value::Float(_) => Value::Float(f32::default()).into(),
                Value::Int(_) => Value::Int(i32::default()).into(),
                Value::Bool(_) => Value::Bool(bool::default()).into(),
                Value::FloatRange(_, min, max) => {
                    Value::FloatRange(f32::default(), *min, *max).into()
                }
                Value::IntRange(_, min, max) => Value::IntRange(i32::default(), *min, *max).into(),
                Value::TextBuffer(_) => Value::TextBuffer(String::new()).into(),
            },
            AttributeValue::Map(m) => AttributeValue::Map(m.clone()),
            AttributeValue::Error(msg) => AttributeValue::Error(msg.clone()),
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }

    pub fn input(label: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        match value {
            AttributeValue::Literal(v) => match v {
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
                Value::Bool(bool) => {
                    ui.set_next_item_width(130.0);
                    ui.checkbox(label, bool);
                }
                Value::FloatRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(label, min.clone(), max.clone()).build(ui, v);
                }
                Value::IntRange(v, min, max) => {
                    ui.set_next_item_width(130.0);
                    imgui::Slider::new(label, min.clone(), max.clone()).build(ui, v);
                }
            },
            AttributeValue::Map(map) => {
                ui.spacing(); 
                for (name, value) in map {
                    let nested = format!("{}/{}", label, name);
                    ui.spacing();
                    AttributeValue::input(nested.to_string(), ui, value);
                }
            },
            _ => (),
        }
    }

    pub fn select(label: String, ui: &imgui::Ui, value: &mut AttributeValue) {
        if let AttributeValue::Map(map) = value {
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
        fn(state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue>,
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
        fn(state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue>,
        Option<AttributeValue>,
        Option<imnodes::OutputPinId>,
        Option<imnodes::AttributeId>,
    ),
}

impl Hash for NodeResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeResource::Title(s) => s.hash(state),
            NodeResource::Input(_, Some(input_id)) => input_id.hash(state),
            NodeResource::Output(_, _, Some(v), Some(output_id)) => {
                output_id.hash(state);
                v.hash(state);
            }
            NodeResource::Attribute(_, _, Some(v), Some(id)) => {
                v.hash(state);
                id.hash(state);
            }
            NodeResource::OutputWithAttribute(_, _, _, Some(v), Some(output_id), Some(attr_id)) => {
                v.hash(state);
                output_id.hash(state);
                attr_id.hash(state)
            }
            _ => {}
        }
    }
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

            let dictionary = NodeResource::index_node_state_to_map(editor_resource);
            nodeid_to_dictionary.extend(dictionary);
        }

        // Merge the attributes from
        for ((output_name, output_pin_id), (input_name, input_pin_id)) in links {
            // This looks like a lot but it's pretty straight-forward
            match match match outputid_to_nodeid_index.get(output_pin_id) {
                // First check to see if we have the output node
                Some(output_node_id) => match &nodeid_to_dictionary.get(output_node_id) {
                    Some(AttributeValue::Map(output_values)) => Some(output_values),
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
                        Some(input_node_id) => match &nodeid_to_dictionary.get(input_node_id) {
                            Some(AttributeValue::Map(input_values)) => {
                                let mut updated_input_values = input_values.clone();
                                updated_input_values
                                    .insert(input_name.to_string(), output_val.clone());

                                nodeid_to_dictionary.insert(
                                    *input_node_id,
                                    AttributeValue::Map(updated_input_values),
                                );
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                _ => (),
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
    fn index_node_state_to_map(
        editor_resource: &EditorResource,
    ) -> HashMap<imnodes::NodeId, AttributeValue> {
        let mut index: HashMap<imnodes::NodeId, AttributeValue> = HashMap::new();
        let mut attribute_dictionary: BTreeMap<String, AttributeValue> = BTreeMap::new();

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

            index.insert(*id, AttributeValue::Map(attribute_dictionary));
        }

        index
    }

    pub fn get_hash_code(editor_resource: &EditorResource) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        editor_resource.hash(&mut hasher);

        hasher.finish()
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

    pub fn copy_blank(&self) -> Self {
        match self {
            NodeResource::Title(v) => NodeResource::Title(v),
            NodeResource::Input(n, _) => NodeResource::Input(*n, None),
            NodeResource::Output(n, o, Some(v), _) => {
                NodeResource::Output(*n, *o, Some(v.copy_blank()), None)
            }
            NodeResource::Attribute(n, d, Some(v), _) => {
                NodeResource::Attribute(*n, *d, Some(v.copy_blank()), None)
            }
            NodeResource::OutputWithAttribute(n, d, o, Some(v), _, _) => {
                NodeResource::OutputWithAttribute(*n, *d, *o, Some(v.copy_blank()), None, None)
            }
            NodeResource::Output(n, o, v, _) => NodeResource::Output(*n, *o, v.clone(), None),
            NodeResource::Attribute(n, d, v, _) => NodeResource::Attribute(*n, *d, v.clone(), None),
            NodeResource::OutputWithAttribute(n, d, o, v, _, _) => {
                NodeResource::OutputWithAttribute(*n, *d, *o, v.clone(), None, None)
            }
        }
    }
}

impl Node for NodeResource {
    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui) {
        match self {
            NodeResource::Title(title) => node.add_titlebar(|| ui.text(format!("{}\t\t", title))),
            NodeResource::Input(name, Some(id)) => {
                let name = name();
                ui.set_next_item_width(130.0);
                node.add_input(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Output(name, _, Some(_), Some(id)) => {
                let name = name();
                ui.set_next_item_width(130.0);
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
                if let Some(attr) = attr {
                    let name = name();
                    node.attribute(attr_id.clone(), || {
                        display(name.to_string(), &ui, attr);
                    });
                    node.add_output(output_id.clone(), imnodes::PinShape::Circle, || {
                        ui.text(name);
                    });
                }
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
                NodeResource::Attribute(name, display, v, None) => {
                    NodeResource::Attribute(name, display, v, Some(id_gen.next_attribute()))
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

#[derive(Clone, Hash)]
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

impl EditorResource {
    pub fn copy_blank(&self, new_id: Option<imnodes::NodeId>) -> EditorResource {
        match self {
            EditorResource::Node { resources, .. } => EditorResource::Node {
                id: new_id,
                resources: resources.iter().map(|f| f.copy_blank()).collect(),
            },
            _ => panic!("Cannot get blank copies of links"),
        }
    }
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
                EditorResource::Node {
                    id: Some(existing),
                    resources,
                } => EditorResource::Node {
                    id: Some(*existing),
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
                    next.show(node_scope, ui);
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

    fn context_menu(&mut self, _: &imgui::Ui) {}
}
