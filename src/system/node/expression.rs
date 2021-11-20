use super::{resource::EditorResource, NodeResource};
use crate::Resource;
use crate::system::Value;
use specs::prelude::*;
use specs::DenseVecStorage;

fn expression(name: &'static str) -> Vec<NodeResource> {
    vec![
        NodeResource::Title(name),
        NodeResource::Input(|| "lhs", None),
        NodeResource::Input(|| "rhs", None),
        NodeResource::Output(
            || "sum",
            |state| {
                let lhs = state.get("lhs");
                let rhs = state.get("rhs");

                let lhs = match lhs {
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

                let rhs = match rhs {
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
            },
            None,
            None,
        ),
    ]
}

// fn index_state(state: Vec<EditorResource>) -> HashMap<String, AttributeValue> {
//     // input -> nodeid
//     let mut idx: HashMap<(String, InputPinId), NodeId> = std::collections::HashMap::new();
//     state.iter().for_each(|r| {
//         if let EditorResource::Node {
//             resources,
//             id: Some(node_id),
//         } = r {
//             resources.iter().for_each(|r| {
//                 if let NodeResource::Input(name, Some(input_id)) = r {
//                     idx.insert((name().to_string(), *input_id), *node_id);
//                 }
//             })
//         }
//     });

//     idx
// }

pub struct Sum(EditorResource);

impl Default for Sum {
    fn default() -> Self {
        Sum(EditorResource::Node {
            resources: expression("Add"),
            id: None,
        })
    }
}

impl Into<EditorResource> for Sum {
    fn into(self) -> EditorResource {
        self.0
    }
}

impl Component for Sum {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Expression<T> {
    lhs: T,
    rhs: T,
}

impl<T> Resource for Expression<T> {
    type Visitor = fn(&T, &T) -> T;
    type Value = T;

    // Expressions accept visitors who are functions that receive two immutable values
    // and return a value of the same type
    fn accept(&self, visitor: Self::Visitor) -> Option<Self::Value> {
        Some(visitor(&self.lhs, &self.rhs))
    }

    // An expression does not have side-effects therefore it returns None
    // from accept_mut
    fn accept_mut(&mut self, _: Self::Visitor) -> Option<Self::Value> {
        None
    }
}

#[test]
fn test_simple() {
    let sum = Expression::<i32> { lhs: 10, rhs: 10 };

    let result = sum.accept(|lhs, rhs| lhs + rhs);

    assert_eq!(Some(20), result);
}

#[test]
fn test_complex() {
    // Testing recursive expressoin logic
    let complex_sum = Expression::<Expression<i32>> {
        lhs: Expression::<i32> { lhs: 10, rhs: 10 },
        rhs: Expression::<i32> { lhs: 12, rhs: 14 },
    };

    let result = if let Some(complex_result) = complex_sum.accept(|lhs, rhs| {
        let s = |lhs: &i32, rhs: &i32| lhs + rhs;

        if let (Some(l), Some(r)) = (lhs.accept(s), rhs.accept(s)) {
            return Expression::<i32> { lhs: l, rhs: r };
        }

        return Expression::<i32> {
            lhs: i32::default(),
            rhs: i32::default(),
        };
    }) {
        complex_result.accept(|lhs, rhs| lhs + rhs) == Some(46)
    } else {
        false
    };

    assert!(result, "expected ");
}
