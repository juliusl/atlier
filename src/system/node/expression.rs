use super::{AttributeValue, NodeResource};
use crate::{
    system::{EditorResource, Value},
    Resource,
};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct Expression {
    lhs: AttributeValue,
    rhs: AttributeValue,
}

impl Expression {
    pub fn new_add_node() -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Add"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(
                    || "sum",
                    Expression::sum,
                    None,
                    None,
                ),
            ],
            id: None,
        }
    }

    fn sum(state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue> {
        let lhs = match state.get("lhs") {
            Some(v) => match v {
                crate::system::AttributeValue::Literal(l) => match l {
                    crate::system::Value::Float(f) => *f,
                    crate::system::Value::Int(i) => *i as f32,
                    _ => 0.0,
                },
                _ => 0.0,
            },
            None => 0.00,
        };

        let rhs = match state.get("rhs") {
            Some(v) => match v {
                crate::system::AttributeValue::Literal(l) => match l {
                    crate::system::Value::Float(f) => *f,
                    crate::system::Value::Int(i) => *i as f32,
                    _ => 0.0,
                },
                _ => 0.0,
            },
            None => 0.00,
        };

        let sum = lhs + rhs;
        if sum > 0.0 {
            Some(Value::Float(sum).into())
        } else {
            None
        }
    }
}

impl Resource for Expression {
    type Visitor = fn(state: &HashMap<String, AttributeValue>) -> Option<AttributeValue>;
    type Value = AttributeValue;

    // Expressions accept visitors who are functions that receive two immutable values
    // and return a value of the same type
    fn accept(&self, visitor: Self::Visitor) -> Option<Self::Value> {
        let mut state: HashMap<String, AttributeValue> = HashMap::new();

        state.insert("lhs".to_string(), self.lhs.clone());
        state.insert("rhs".to_string(), self.rhs.clone());

        visitor(&state)
    }

    // An expression does not have side-effects therefore it returns None
    // from accept_mut
    fn accept_mut(&mut self, _: Self::Visitor) -> Option<Self::Value> {
        None
    }
}

