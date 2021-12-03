use std::{collections::BTreeMap, any::Any};

use super::{Value, State, NodeExterior, EditorResource, NodeResource, Reducer};

#[derive(Debug, Clone, Hash)]
pub enum Attribute {
    Literal(Value),
    Map(BTreeMap<String, Attribute>),
    Empty,
    Error(String),
}

impl Into<EditorResource> for Attribute {
    fn into(self) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title(Self::title()),
                self.into(),
                <Attribute as Reducer>::resource(),
            ],
            id: None
        }
    }
}

impl Into<NodeResource> for Attribute {
    fn into(self) -> NodeResource {
        NodeResource::Attribute(
            Self::param_name, 
            Self::input, 
            Some(self), 
            None)
    }
}

impl NodeExterior for Attribute {
    fn title() -> &'static str {
        "Attribute"
    }

    fn group_name() -> &'static str {
        "System"
    }
}

impl Reducer for Attribute {
    fn result_name() -> &'static str {
        "attribute_value"
    }

    fn param_name() -> &'static str {
        "attribute"
    }

    fn reduce(attribute: Option<Attribute>) -> Option<Attribute> {
        attribute
    }
}

impl Into<f32> for Attribute {
    fn into(self) -> f32 {
        match self {
            crate::system::Attribute::Literal(l) => match l {
                crate::system::Value::Float(f) => f,
                crate::system::Value::Int(i) => (i as f32),
                crate::system::Value::FloatRange(f, _, _) => f,
                crate::system::Value::IntRange(i, _, _) => (i as f32),
                _ => 0.00
            },
            _ => 0.00
        }
    }
}

impl Into<f64> for Attribute {
    fn into(self) -> f64 {
       let v: f32 = self.into();
       v as f64
    }
}

impl From<f32> for Attribute {
    fn from(f: f32) -> Self {
        Attribute::Literal(Value::Float(f))
    }
}

impl From<f64> for Attribute {
    fn from(f: f64) -> Self {
        Attribute::Literal(Value::Float(f as f32))
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

impl From<&str> for Attribute {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<&BTreeMap<String, Attribute>> for Attribute {
    fn from(m: &BTreeMap<String, Attribute>) -> Self {
        Attribute::Map(m.to_owned())
    }
}

impl From<State> for Attribute {
    fn from(s: State) -> Self {
        Attribute::Map(s.into())
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
