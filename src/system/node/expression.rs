use specs::{Entities, System};

use super::{Attribute, NodeResource, Output, visitor::{ NodeExterior, NodeInterior, NodeVisitor}};
use crate::{system::{State, Value}};
use std::{marker::PhantomData};

// An ExpressionFunc takes two parameters of the same type and returns a result of the same type
pub trait ExpressionFunc2<T> {
    // "call by name"
    fn title() -> fn()->&'static str; 
    fn result_name() -> fn()->&'static str; 
    fn func() -> fn(T, T) -> T;
}

// FloatExpression is a data structure of float parameters for a receiving ExpresionFunc
#[derive(Clone)]
pub struct FloatExpression<V> 
where
    V: ExpressionFunc2<f32>
{
    lhs: f32,
    rhs: f32,
    _p: PhantomData<V>,
}

impl<V> From<State> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32>
{
    fn from(state: State) -> Self {
       match (state.get("lhs"), state.get("rhs")) {
           (Some(lhs), Some(rhs)) => FloatExpression { lhs: lhs.into(), rhs: rhs.into(), _p: PhantomData::default() },
           (Some(lhs), None) => FloatExpression { lhs: lhs.into(), rhs: 0.0, _p: PhantomData::default() },
           (None, Some(rhs)) => FloatExpression { lhs: 0.0, rhs: rhs.into(), _p: PhantomData::default() },
           _ => FloatExpression { lhs: 0.00, rhs: 0.00, _p: PhantomData::default() }
       }
    }
}

// Implementing this adds an accept fn() to accept and evaluate visitors
impl<'a, V> NodeInterior<'a> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32> + Clone
{
    type Visitor = FloatExpression<V>;
}

impl<'a, V> System<'a> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32>
{
    type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}


impl <V> Default for FloatExpression<V>
where
    V: ExpressionFunc2<f32>
{
    fn default() -> Self {
        FloatExpression::<V> {
            lhs: 0.00,
            rhs: 0.00,
            _p: PhantomData::default()
        }
    }
}

// Implementing NodeExterior adds methods to return a EditorResource::Node
impl<V> NodeExterior for FloatExpression<V>  
where
    V: ExpressionFunc2<f32> 
{    
    fn title() -> &'static str {
        V::title()()
    }

    fn group_name() -> &'static str {
        "Float Expressions"
    }

    fn inputs() -> Option<Vec<NodeResource>> {
        Some(vec![
            NodeResource::Input(||"lhs", None),
            NodeResource::Input(||"rhs", None)
        ])
    }
}

/// By Implmenting `Output`, `NodeVisitor`, and `NodeInterior` the EditorResource can be generated
/// This allows you to implement several different implementations of generic type `V` without needing to reimplement EditorResource methods for each variant
impl<'a, V> Output<'a> for FloatExpression<V>  
where
    V: ExpressionFunc2<f32> + Clone
{
    fn output_name() -> &'static str {
        V::result_name()()
    }
}

impl<V> NodeVisitor for FloatExpression<V> 
where
    V: ExpressionFunc2<f32> + Clone
{
    // Evaluate the result of the visitor 
    fn evaluate(&self) -> Option<State> {
        Some(Attribute::Literal(Value::Float(V::func()(self.lhs, self.rhs))).into())
    }

    fn call(&self, _: &str) -> Self {
        self.clone()
    }
}

impl<V> Into<State> for FloatExpression<V> 
where
    V: ExpressionFunc2<f32> + Clone 
{
    fn into(self) -> State {
        State::default()
            .insert("lhs", Attribute::from(self.lhs))
            .insert("rhs", Attribute::from(self.rhs))
            .snapshot()
    }
}


/// The `Add` node can output the sum of the `lhs` and `rhs` inputs
#[derive(Clone)]
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

/// The `Divide` node can output the quotient of the `lhs` and `rhs` inputs
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

/// The `Subtract` node can output the difference of the `lhs` and `rhs` inputs
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

/// The `Multiply` node can output the product of the `lhs` and `rhs` inputs
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
