use crate::system::{EditorResource, NodeExterior, NodeResource, Reducer, Value};

pub struct Sine;

impl NodeExterior for Sine {
    fn title() -> &'static str {
        "Sine Function"
    }

    fn resource(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource {
        EditorResource::Node{
            id: nodeid,
            resources: vec![
                NodeResource::Title(Self::title()),
                NodeResource::Input(Self::param_name, None),
                NodeResource::Reducer(Self::result_name, |_, _, _, _|{}, Self::map, Self::reduce, (0, None), None, None),
            ],
        }
    }
}

impl Reducer for Sine {
    fn param_name() -> &'static str {
        "input"
    }

    fn result_name() -> &'static str {
        "sin(input)"
    }

    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
       if let Some(attr) = attribute {
          let value = match attr {
            crate::system::Attribute::Literal(l) => match l {
                crate::system::Value::Float(f) => f.sin(),
                crate::system::Value::Int(i) => (i as f32).sin(),
                crate::system::Value::FloatRange(f, _, _) => f.sin(),
                crate::system::Value::IntRange(i, _, _) => (i as f32).sin(),
                _ => 0.00
            },
            _ => 0.00
            };

            Some(Value::Float(value).into())
        } else {
            None
        }
    }
}