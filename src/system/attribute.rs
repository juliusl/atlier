use std::collections::BTreeMap;
use specs::{Component, VecStorage};

use super::{NodeExterior, Value};

pub struct Resource<T>
where 
    T: NodeExterior + 'static + Sync
{    
    state: BTreeMap<String, Attribute>,
    exterior: Option<T>
}

impl<T> Default for Resource<T> 
where 
    T: NodeExterior + 'static + Sync
{
    fn default() -> Self {
        Self { state: BTreeMap::new(), exterior: None }
    }
}

impl<T> Resource<T> 
where 
    T: NodeExterior + 'static + Sync
{
    pub fn new(t: T) -> Self {
        Self {
            exterior: Some(t),
            state: BTreeMap::new()
        }
    }
}

impl<T> NodeExterior for Resource<T>
where 
    T: NodeExterior + 'static + Sync + Send
{
    fn resource(nodeid: Option<imnodes::NodeId>) -> super::EditorResource {
        T::resource(nodeid)
    }

    fn title() -> &'static str {
        T::title()
    }
}

impl<T> Component for Resource<T>
where 
    T: NodeExterior + 'static + Sync + Send
{
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone, Hash)]
pub enum Attribute {
    Literal(Value),
    Map(BTreeMap<String, Attribute>),
    Empty,
    Error(String),
}

impl From<f32> for Attribute {
    fn from(f: f32) -> Self {
        Attribute::Literal(Value::Float(f))
    }
}

impl From<i32> for Attribute {
    fn from(i: i32) -> Self {
        Attribute::Literal(Value::Int(i))
    }
}

impl From<bool> for Attribute {
    fn from(b: bool) -> Self {
        Attribute::Literal(Value::Bool(b))
    }
}

impl From<[f32; 3]> for Attribute {
    fn from(fr: [f32; 3]) -> Self {
        Attribute::Literal(Value::FloatRange(fr[0], fr[1], fr[2]))
    }
}

impl From<[i32; 3]> for Attribute {
    fn from(ir: [i32; 3]) -> Self {
        Attribute::Literal(Value::IntRange(ir[0], ir[1], ir[2]))
    }
}

impl From<String> for Attribute {
    fn from(s: String) -> Self {
        Attribute::Literal(Value::TextBuffer(s))
    }
}

impl From<&BTreeMap<String, Attribute>> for Attribute {
    fn from(m: &BTreeMap<String, Attribute>) -> Self {
        Attribute::Map(m.to_owned())
    }
}

impl Attribute {

    // Get a blank copy
    pub fn copy_blank(&self) -> Self {
        match self {
            Attribute::Literal(l) => match l {
                Value::Float(_) => Attribute::from(f32::default()),
                Value::Int(_) => Attribute::from(i32::default()),
                Value::Bool(_) => Attribute::from(bool::default()),
                Value::FloatRange(_, min, max) => {
                    Attribute::from([f32::default(), *min, *max])
                }
                Value::IntRange(_, min, max) => Attribute::from([i32::default(), *min, *max]),
                Value::TextBuffer(_) => Attribute::from(String::new()),
            },
            Attribute::Map(_) => Attribute::from(&BTreeMap::new()),
            Attribute::Error(msg) => Attribute::Error(msg.clone()),
            Attribute::Empty => Attribute::Empty,
        }
    }
}
