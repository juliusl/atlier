use super::{AttributeValue, NodeResource};
use crate::system::{EditorResource, Value};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Expression {
    lhs: AttributeValue,
    rhs: AttributeValue,
}

pub enum Visitor
{
    ExpressionFloat2(String, String, fn(f32, f32) -> f32),
    ExpressionInt2(String, String, fn(i32, i32) -> i32),
}

impl Visitor
{
    pub fn evaluate(&self, state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue> {
        match self.clone() {
            Visitor::ExpressionFloat2(lhs, rhs, expr) => {
                let lhs = match state.get(lhs) {
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
        
                let rhs = match state.get(rhs) {
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
        
                Some(AttributeValue::Literal(Value::Float(expr(lhs, rhs))))
            },
            Visitor::ExpressionInt2(_, _, _) => todo!(),
        }
    }
}

impl Expression {
    pub fn new_add_node(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Add"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "sum", |state| {
                    let visitor = Visitor::ExpressionFloat2("lhs".to_string(), "rhs".to_string(), Expression::sum);
                    visitor.evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_multiply_node(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Multiply"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "product", |state| {
                    let visitor = Visitor::ExpressionFloat2("lhs".to_string(), "rhs".to_string(), Expression::product);
                    visitor.evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_divide_node(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Divide"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "quotient", |state| {
                    let visitor = Visitor::ExpressionFloat2("lhs".to_string(), "rhs".to_string(), Expression::quotient);
                    visitor.evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_subtract_node(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Subtract"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "difference", |state| {
                    let visitor = Visitor::ExpressionFloat2("lhs".to_string(), "rhs".to_string(), Expression::difference);
                    visitor.evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }
    
    fn sum(lhs: f32, rhs: f32) -> f32 {
        lhs + rhs
    }
    fn product(lhs: f32, rhs: f32) -> f32 {
        lhs * rhs
    }
    fn quotient(lhs: f32, rhs: f32) -> f32 { 
        lhs / rhs
    }
    fn difference(lhs: f32, rhs: f32) -> f32 { 
        lhs - rhs
    }
}

