use crate::system::{Attribute, NodeExterior, NodeResource, Reducer, Value};

pub struct Time;

impl NodeExterior for Time {
    fn title() -> &'static str {
        "Time"
    }

    fn group_name() -> &'static str {
        "Functions"
    }
}

impl Reducer for Time {
    fn param_name() -> &'static str {
        "enabled"
    }

    fn result_name() -> &'static str {
        "seconds from epoch"
    }

    fn parameter() -> NodeResource {
        NodeResource::Attribute(
            Self::param_name, 
            Self::input, 
            Some(false.into()), 
            None)
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