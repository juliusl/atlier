use crate::system::{Attribute, EditorResource, NodeExterior, NodeResource, Reducer, Value};

pub struct Time;

impl NodeExterior for Time {
    fn title() -> &'static str {
        "Time"
    }

    fn resource(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource {
        EditorResource::Node {
            id: nodeid,
            resources: vec![
                NodeResource::Title("Time"),
                NodeResource::Output(
                    Time::result_name,
                    |_| {
                        let value = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        % 1000) as f32
                        / 1000.0;

                        Some(Value::Float(value).into())
                    },
                    Some(Value::Float(0.00).into()),
                    None,
                ),
            ],
        }
    }
}

impl Reducer for Time {
    fn param_name() -> &'static str {
        "enabled"
    }

    fn result_name() -> &'static str {
        "seconds from epoch"
    }

    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        if let Some(Attribute::Literal(Value::Bool(enabled))) = attribute {
            if !enabled {
                return None 
            }

            if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Some(Value::Float(dur.as_secs_f32()).into())
            } else {
                None
            }
        } else {
            None 
        }
    }
}