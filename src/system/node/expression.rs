use super::{AttributeValue, NodeResource, visitor::{ NodeExterior, NodeInterior, NodeVisitor}};
use crate::{system::{EditorResource, Value}};
use std::{collections::BTreeMap, marker::PhantomData};

// An ExpressionFunc takes two parameters of the same type and returns a result of the same type
pub trait ExpressionFunc2<T> {
    // "call by name"
    fn title() -> fn()->&'static str; 
    fn result_name() -> fn()->&'static str; 
    fn func() -> fn(T, T) -> T;
}

// FloatExpression is a data structure of float parameters for a receiving ExpresionFunc
pub struct FloatExpression<V> 
where
    V: ExpressionFunc2<f32>
{
    lhs: f32,
    rhs: f32,
    _p: PhantomData<V>,
}

impl<V> NodeExterior for FloatExpression<V>  
where
    V: ExpressionFunc2<f32> 
{
    // Defintion for an editor resource
    fn resource(nodeid: Option<imnodes::NodeId>) -> EditorResource {
            EditorResource::Node {
                resources: vec!(
                    NodeResource::Title(V::title()()),
                    NodeResource::Input(|| "lhs", None), 
                    NodeResource::Input(|| "rhs", None), 
                    NodeResource::Output(V::result_name(), |state| {
                        // NodeInterior.accept(state) -> NodeVisitor.evaluate -> output 
                        FloatExpression::<V>::accept(state).evaluate() 
                    }, 
                    None,
                    None)
                ),
                id: nodeid
            }
    }
}

impl<'a, V> NodeInterior<'a> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32> 
{
    type Literal = (Option<&'a AttributeValue>, Option<&'a AttributeValue>); 
    type Visitor = FloatExpression<V>;

    // If this interior is implemented there are two inputs "lhs" and "rhs"
    fn accept(state: &'a BTreeMap<String, AttributeValue>) -> Self::Visitor {
         Self::Visitor::from((state.get("lhs"),  state.get("rhs")))
    }
}

impl<V> NodeVisitor for FloatExpression<V> 
where
    V: ExpressionFunc2<f32> 
{
    // Evaluate the result of the visitor 
    fn evaluate(&self) -> Option<AttributeValue> {
        Some(AttributeValue::Literal(Value::Float(V::func()(self.lhs, self.rhs))))
    }
}

// TODO: This is in a good spot to be converted to a derive macro  
pub struct Add;
impl ExpressionFunc2<f32> for Add {
    fn title() -> fn()->&'static str {
        ||"Add"
    }

    fn result_name() -> fn()->&'static str {
        ||"sum"
    }

    fn func() -> fn(f32, f32) -> f32 {
        |l, r| l + r 
    }
}

pub struct Divide; 
impl ExpressionFunc2<f32> for Divide {
    fn title() -> fn()->&'static str {
        || "Divide"
    }

    fn result_name() -> fn()->&'static str {
        || "quotient"
    }

    fn func() -> fn(f32, f32) -> f32 {
        |l, r| l/r
    }
}

pub struct Subtract;
impl ExpressionFunc2<f32> for Subtract {
    fn title() -> fn()->&'static str {
        || "Subtract"
    }

    fn result_name() -> fn()->&'static str {
        || "difference"
    }

    fn func() -> fn(f32, f32) -> f32 {
        |l, r| l - r 
    }
}

pub struct Multiply;
impl ExpressionFunc2<f32> for Multiply {
    fn title() -> fn()->&'static str {
        || "Multiply"
    }

    fn result_name() -> fn()->&'static str {
        || "product"
    }

    fn func() -> fn(f32, f32) -> f32 {
        |l, r| l*r
    }
}

impl<'a, V> From<(Option<&'a AttributeValue>, Option<&'a AttributeValue>)> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32>
{
    fn from(tuple: (Option<&'a AttributeValue>, Option<&'a AttributeValue>)) -> Self {
       match tuple {
           (Some(AttributeValue::Literal(Value::Float(lhs))), Some(AttributeValue::Literal(Value::Float(rhs)))) => FloatExpression { lhs: *lhs, rhs: *rhs, _p: PhantomData::default() },
           (Some(AttributeValue::Literal(Value::Float(lhs))), None) => FloatExpression { lhs: *lhs, rhs: 0.0, _p: PhantomData::default() },
           (None, Some(AttributeValue::Literal(Value::Float(rhs)))) => FloatExpression { lhs: 0.0, rhs: *rhs, _p: PhantomData::default() },
           _ => FloatExpression { lhs: 0.00, rhs: 0.00, _p: PhantomData::default() }
       }
    }
}