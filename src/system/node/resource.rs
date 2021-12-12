use super::{NodeComponent, EditorComponent};
use crate::system::{Attribute, Routine, State};
use imnodes::{InputPinId, OutputPinId};
use std::{collections::{BTreeMap, HashMap}, hash::{Hash, Hasher}};

#[derive(Clone)]
pub enum NodeResource {
    Title(&'static str),
    Extension(&'static str),
    Input(fn() -> &'static str, Option<imnodes::InputPinId>),
    Attribute(
        fn() -> &'static str,
        fn(name: String, width: f32, ui: &imgui::Ui, attribute_value: &mut Attribute),
        Option<Attribute>,
        Option<imnodes::AttributeId>,
    ),
    Listener(
        fn() -> &'static str,
        fn(Option<Attribute>) -> Option<Attribute>,
        Option<imnodes::InputPinId>
    ),
    Reducer(
        fn() -> &'static str,
        fn(name: String, width: f32, ui: &imgui::Ui, attribute_value: &mut Attribute),
        fn(state: State) -> (u64, Option<Attribute>),
        fn(attribute: Option<Attribute>) -> Option<Attribute>,
        (u64, Option<Attribute>),
        Option<imnodes::OutputPinId>,
        Option<imnodes::AttributeId>,
    ),
    Output(
        fn() -> &'static str,
        fn(state: State) -> Option<Attribute>,
        Option<Attribute>,        
        Option<imnodes::OutputPinId>,
    ),
    Event(
        fn() -> &'static str,
        fn(name:String, ui: &imgui::Ui) -> bool,
        fn(state: State) -> Option<State>,
        fn(state: State) -> Option<Attribute>,
        Option<Attribute>,
        Option<imnodes::OutputPinId>,
    ),
    Display(
        fn() -> &'static str,
        fn(name: String, width: f32, ui: &imgui::Ui, state: &State),
        Option<imnodes::AttributeId>,
    ),
    Empty
}

impl From<State> for NodeResource {
    fn from(state: State) -> Self {
        if let Some(Attribute::OrderedMap(map)) = state.get("Title") {
            let resource = if let Some(Attribute::Functions(Routine::Name(name))) = map.get("Name") {
                NodeResource::Title(name())
            } else {
                NodeResource::Empty
            };

            return resource;
        } 

        todo!()
    }
}

impl Hash for NodeResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeResource::Title(s) => s.hash(state),
            NodeResource::Extension(s) => s.hash(state),
            NodeResource::Input(_, Some(input_id)) => input_id.hash(state),
            NodeResource::Output(_, _, Some(v), Some(output_id)) => {
                output_id.hash(state);
                v.hash(state);
            }
            NodeResource::Attribute(_, _, Some(v), Some(id)) => {
                v.hash(state);
                id.hash(state);
            }
            NodeResource::Reducer(_, _, _, _, (hash, Some(..)), Some(output_id), Some(attr_id)) => {
                hash.hash(state);
                output_id.hash(state);
                attr_id.hash(state);
            }
            NodeResource::Display(_, _, Some(attr_id)) => {
                attr_id.hash(state);
            },
            NodeResource::Event(_, _, _, _, Some(b), Some(o)) => {
                b.hash(state);
                o.hash(state);
            },
            NodeResource::Listener(_, _, Some(i)) => {
                i.hash(state);
            },
            _ => {}
        }
    }
}

impl NodeResource {
    pub fn index_editor_state(resources: Vec<EditorResource>) -> HashMap<imnodes::NodeId, Attribute> {
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
        let mut nodeid_to_dictionary: HashMap<imnodes::NodeId, Attribute> = HashMap::new();
        for editor_resource in resources.iter() {
            let inputs_index = NodeResource::index_node_inputs(editor_resource);
            inputid_to_nodeid_index.extend(inputs_index);

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
                    Some(Attribute::OrderedMap(output_values)) => Some(output_values),
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
                            Some(Attribute::OrderedMap(input_values)) => {
                                let mut updated_input_values = input_values.clone();
                                updated_input_values
                                    .insert(input_name.to_string(), output_val.clone());

                                nodeid_to_dictionary.insert(
                                    *input_node_id,
                                    Attribute::OrderedMap(updated_input_values),
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
                NodeResource::Input(_, Some(input_id)) | 
                NodeResource::Listener(_, _, Some(input_id)) => Some(input_id),
                _ => None,
            }) {
                index.insert(*r, *id);
            }
        }
        index
    }


    fn index_node_outputs( editor_resource: &EditorResource) -> HashMap<OutputPinId, imnodes::NodeId> {
        let mut index: HashMap<OutputPinId, imnodes::NodeId> = HashMap::new();

        if let EditorResource::Node {
            id: Some(id),
            resources,
        } = editor_resource
        {
            for r in resources.iter().filter_map(|f| match f {
                NodeResource::Output(_, _, Some(..), Some(output_id)) |
                NodeResource::Reducer(_, _, _, _, (_, Some(..)), Some(output_id), ..) | 
                NodeResource::Event(_, _, _, _, _, Some(output_id)) => {
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
    fn index_node_state_to_map(editor_resource: &EditorResource) -> HashMap<imnodes::NodeId, Attribute> {
        let mut index: HashMap<imnodes::NodeId, Attribute> = HashMap::new();
        let mut attribute_dictionary: BTreeMap<String, Attribute> = BTreeMap::new();

        if let EditorResource::Node {
            id: Some(id),
            resources,
        } = editor_resource
        {
            resources.iter().for_each(|f| match f {
                NodeResource::Attribute(name, _, Some(v), _) | 
                NodeResource::Event(name, _, _, _, Some(v), _)  |  
                NodeResource::Output(name, _, Some(v), _) | 
                NodeResource::Reducer(name, _, _, _, (_, Some(v)), _, _)
                => {
                    attribute_dictionary.insert(name().to_string(), v.clone());
                }
                NodeResource::Title(title) => { 
                    attribute_dictionary.insert("title".to_string(), Attribute::from(title.to_string()));
                }
                NodeResource::Extension(ext_title) => {
                    attribute_dictionary.insert("ext_title".to_string(), Attribute::from(ext_title.to_string()));
                }

                NodeResource::Input(name, Some(..)) |  NodeResource::Listener(name, _, Some(..)) => {
                    attribute_dictionary.insert(name().to_string(), Attribute::Empty);
                }
                _ => {}
            });

            index.insert(*id, Attribute::OrderedMap(attribute_dictionary));
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
            NodeResource::Extension(s) | NodeResource::Title(s) => s.to_string(),
            NodeResource::Input(s, _)
            | NodeResource::Output(s, ..)
            | NodeResource::Attribute(s, _, _, _)
            | NodeResource::Display(s,  _, _)
            | NodeResource::Reducer(s, _, _, _, _, _, _)
            | NodeResource::Event(s, _, _, _, _, _)
            | NodeResource::Listener(s, _, _) => s().to_string(),
            NodeResource::Empty => todo!(),
        }
    }

    pub fn debug_state(&self) -> String {
        match self {
            NodeResource::Extension(_) | NodeResource::Title(_) => String::new(),
            NodeResource::Input(_, v) => format!("{:#?}", v),
            NodeResource::Output(_, _, o, v) => format!("{:#?} {:#?}", o, v),
            NodeResource::Attribute(_, _, v, _) => format!("{:#?}", v),
            NodeResource::Display(s, _, i) => format!("{:#?} {:#?}", s(), i),
            NodeResource::Reducer(s, _, _, _, v, o, a) => {
                format!("{:#?} {:#?} {:#?} {:#?}", s(), v, o, a)
            }
            NodeResource::Event(s, _, _, _, v, o) => format!("{:#?} {:#?} {:#?}", s(), v, o),
            NodeResource::Listener(s, _, i) => format!("{:#?} {:#?}", s(), i),
            NodeResource::Empty => todo!(),
        }
    }

    pub fn copy_blank(&self) -> Self {
        match self {
            NodeResource::Extension(v) => NodeResource::Extension(v),
            NodeResource::Title(v) => NodeResource::Title(v),
            NodeResource::Input(n, _) => NodeResource::Input(*n, None),
            NodeResource::Output(n, o, Some(v), _) => {
                NodeResource::Output(*n, *o, Some(v.copy_blank()), None)
            }
            NodeResource::Attribute(n, d, Some(v), _) => {
                NodeResource::Attribute(*n, *d, Some(v.copy_blank()), None)
            }
            NodeResource::Reducer(n, d, m, r, (_, Some(v)), _, _) => {
                NodeResource::Reducer(*n, *d, *m, *r, (0, Some(v.copy_blank())), None, None)
            }
            NodeResource::Output(n, o, v, _) => NodeResource::Output(*n, *o, v.clone(), None),
            NodeResource::Attribute(n, d, v, _) => NodeResource::Attribute(*n, *d, v.clone(), None),
            NodeResource::Display(n, a, _) => NodeResource::Display(*n, *a, None),
            NodeResource::Reducer(n, d, m, r, v, _, _) => {
                NodeResource::Reducer(*n, *d, *m, *r, (0, v.1.clone()), None, None)
            }
            NodeResource::Event(n, uie, e, t, v, _) => NodeResource::Event(*n,*uie, *e, *t, v.clone(), None),
            NodeResource::Listener(n, i, _) => NodeResource::Listener(*n, *i, None),
            NodeResource::Empty => todo!(),
        }
    }
}

impl NodeComponent for NodeResource {
    fn show(&mut self, node: &mut imnodes::NodeScope, ui: &imgui::Ui, state: &State) {
        let width = 300.0;

        match self {
            NodeResource::Title(title) => node.add_titlebar(|| ui.text(format!("{}\t\t", title))),
            NodeResource::Extension(extention_title) => {
                // TODO: Ideally I can use a seperator here
                ui.new_line();
                ui.text(extention_title.to_string());
            }
            NodeResource::Input(name, Some(id)) => {
                let name = name();
                ui.set_next_item_width(width);
                node.add_input(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Output(name, _, Some(_), Some(id)) => {
                let name = name();
                ui.set_next_item_width(width);
                node.add_output(id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                });
            }
            NodeResource::Attribute(name, display, Some(attr), Some(id)) => {
                let name = name();
                node.attribute(id.clone(), || {
                    display(name.to_string(), width, &ui, attr);
                });
            }
            NodeResource::Reducer(
                name,
                display,
                _,
                _,
                (_, Some(attr)),
                Some(output_id),
                Some(attr_id),
            ) => {
                let name = name();
                node.attribute(attr_id.clone(), || {
                    display(name.to_string(), width, &ui, attr);
                });

                node.add_output(output_id.clone(), imnodes::PinShape::Circle, || {
                    ui.text(name);
                })
            }
            NodeResource::Display(
                name,
                display_action,
                Some(attr_id),
            ) => {
                // An action maintains it's own state from when this node is initialzied
                // It will receive updates for the interior of the node but cannot dispatch any changes, except to it's own state
                node.attribute(*attr_id, || {
                    display_action(name().to_string(), width, &ui, state);
                })
            }
            NodeResource::Event(name,_, _, _, Some(..), Some(o)) => {
                node.add_output(*o, imnodes::PinShape::Quad, ||{
                    ui.text(name());
                })
            }
            NodeResource::Listener(name, _, Some(i)) => {
                node.add_input(*i, imnodes::PinShape::Quad, ||{
                    ui.text(name());
                })
            }
            _ => {}
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
                NodeResource::Reducer(name, display, map, reduce, val, None, None) => {
                    NodeResource::Reducer(
                        name,
                        display,
                        map,
                        reduce,
                        val,
                        Some(id_gen.next_output_pin()),
                        Some(id_gen.next_attribute()),
                    )
                }
                NodeResource::Display(name, action, None) => {
                    NodeResource::Display(name, action, Some(id_gen.next_attribute()))
                }
                NodeResource::Input(name, None) => {
                    NodeResource::Input(name, Some(id_gen.next_input_pin()))
                }
                NodeResource::Event(name, ui_enable, enable, send, v, None) => {
                    NodeResource::Event(name, ui_enable, enable, send, v, Some(id_gen.next_output_pin()))
                },
                NodeResource::Listener(name, listen, None) => {
                    NodeResource::Listener(name, listen, Some(id_gen.next_input_pin()))
                },
                _ => r.clone()
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
    Message {
        id: imnodes::LinkId,
        start: (String, imnodes::OutputPinId),
        end: (String, imnodes::InputPinId),
    }
}

impl EditorResource {
    pub fn merge(&self, ref other: EditorResource) -> EditorResource {
        if let (EditorResource::Node { id, resources }, EditorResource::Node { .. }) = (self, other)
        {
            let mut my_resources = resources.to_vec();
            if let EditorResource::Node { resources, .. } = other {
                for r in resources
                    .iter()
                    .filter_map(|f| match f {
                        NodeResource::Title(t) => Some(NodeResource::Extension(t)),
                        NodeResource::Extension(_)
                        | NodeResource::Input(_, _)
                        | NodeResource::Output(_, _, _, _)
                        | NodeResource::Attribute(_, _, _, _)
                        | NodeResource::Reducer(_, _, _, _, _, _, _)
                        | NodeResource::Display(_, _, _) 
                        | NodeResource::Event(_, _, _, _, _, _)
                        | NodeResource::Listener(_, _, _) => Some(f.copy_blank()),
                        NodeResource::Empty => todo!(),
                    })
                    .map(|r| r.copy_blank())
                {
                    my_resources.push(r);
                }
            }
            EditorResource::Node {
                resources: my_resources,
                id: id.clone(),
            }
        } else {
            self.copy_blank(None)
        }
    }

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

impl EditorComponent for EditorResource {
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

    fn show(&mut self, editor: &mut imnodes::EditorScope, ui: &imgui::Ui, state: Option<&State>) {
        match self {
            EditorResource::Node {
                id: Some(id),
                resources,
            } =>
            {
            editor.add_node(id.clone(), |mut scope| {
                let mut iter = resources.iter_mut();

                while let Some(next) = iter.next() {
                    let node_scope = &mut scope;
                    if let Some(s) = state {
                        next.show(node_scope, ui, s);
                    }
                }
            });
        },
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
