use std::{hash::Hash};

use super::Attribute;

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Int(i32),
    Bool(bool),
    FloatRange(f32, f32, f32),
    IntRange(i32, i32, i32),
    TextBuffer(String),
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Float(f) => f.to_bits().hash(state),
            Value::Int(i) => i.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::FloatRange(f, fm, fmx) => {
                f.to_bits().hash(state);
                fm.to_bits().hash(state);
                fmx.to_bits().hash(state);
            }
            Value::IntRange(i, im, imx) => {
                i.hash(state);
                im.hash(state);
                imx.hash(state);
            }
            Value::TextBuffer(txt) => txt.hash(state),
        };
    }
}

impl Into<Attribute> for Value {
    fn into(self) -> Attribute {
        Attribute::Literal(self)
    }
}