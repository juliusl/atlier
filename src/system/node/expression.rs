use super::{AttributeValue, NodeResource, visitor::NodeVisitor};
use crate::{system::{EditorResource, Value}};
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum ExpressionVisitor
{
    Float(fn(f32, f32) -> f32),
    Int(fn(i32, i32) -> i32),
}

impl From<fn(i32, i32) -> i32> for ExpressionVisitor {
    fn from(f: fn(i32, i32) -> i32) -> Self {
        ExpressionVisitor::Int(f)
    }
}

impl From<fn(f32, f32) -> f32> for ExpressionVisitor {
    fn from(f: fn(f32, f32) -> f32) -> Self {
        ExpressionVisitor::Float(f)
    }
}

impl NodeVisitor for ExpressionVisitor {
    fn evaluate(&self, state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue> {
        match self {
            ExpressionVisitor::Float(expr) => {
                let rhs: f32 = match state.get("rhs") {
                    Some(rhs) => rhs.clone().into(),
                    _ => 0.0,
                };

                let lhs: f32 = match state.get("lhs") {
                    Some(lhs) => lhs.clone().into(),
                    _ => 0.0,
                };
        
        
                Some(AttributeValue::Literal(Value::Float(expr(lhs, rhs))))
            },
            ExpressionVisitor::Int(expr) => {
                let rhs: i32 = match state.get("rhs") {
                    Some(rhs) => rhs.clone().into(),
                    _ => 0,
                };

                let lhs: i32 = match state.get("lhs") {
                    Some(lhs) => lhs.clone().into(),
                    _ => 0,
                };
        
                Some(AttributeValue::Literal(Value::Int(expr(lhs, rhs))))
            },
        }
    }
}

    pub fn new_add_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Add"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "sum", |state| {
                    ExpressionVisitor::Float(|l: f32, r: f32| l + r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_divide_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Divide"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "quotient", |state| {
                    ExpressionVisitor::Float(|l,r| l / r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_multiply_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Multiply"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "product", |state| {
                    ExpressionVisitor::Float(|l,r| l * r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_subtract_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("Subtract"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "difference", |state| {
                    ExpressionVisitor::Float(|l,r| l - r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_add_int_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("integers::Add"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "sum", |state| {
                    ExpressionVisitor::Int(|l,r| l + r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_modulo_int_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("integers::Divide"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "quotient", |state| {
                    ExpressionVisitor::Int(|l,r| l % r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_multiply_int_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("integers::Multiply"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "product", |state| {
                    ExpressionVisitor::Int(|l,r| l * r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }

    pub fn new_subtract_int_node(nodeid: Option<imnodes::NodeId>) -> EditorResource
    {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("integers::Subtract"),
                NodeResource::Input(|| "lhs", None),
                NodeResource::Input(|| "rhs", None),
                NodeResource::Output(|| "difference", |state| {
                    ExpressionVisitor::Int(|l,r| l - r).evaluate(state)
                }, None, None),
            ],
            id: nodeid,
        }
    }