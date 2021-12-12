use crate::prelude::State;

use super::Attribute;

#[derive(Debug, Clone, Hash)]
pub enum Routine {
    Name(fn() -> &'static str),
    Select(fn(state: State) -> (u64, Option<Attribute>)),
    Reduce(fn(attribute: Option<Attribute>) -> Option<Attribute>),
    Transform(fn(state: State) -> Option<Attribute>),
    Next(fn(state: State) -> Option<State>),
}

impl From<fn() -> &'static str> for Routine {
    fn from(f: fn() -> &'static str) -> Self {
        Routine::Name(f)
    }
}

impl From<fn(state: State) -> (u64, Option<Attribute>)> for Routine {
    fn from(f: fn(state: State) -> (u64, Option<Attribute>)) -> Self {
        Routine::Select(f)
    }
}

impl From<fn(attribute: Option<Attribute>) -> Option<Attribute>> for Routine {
    fn from(f: fn(attribute: Option<Attribute>) -> Option<Attribute>) -> Self {
        Routine::Reduce(f)
    }
}

impl From<fn(state: State) -> Option<Attribute>> for Routine {
    fn from(f: fn(state: State) -> Option<Attribute>) -> Self {
        Routine::Transform(f)
    }
}

impl From<fn(state: State) -> Option<State>> for Routine {
    fn from(f: fn(state: State) -> Option<State>) -> Self {
        Routine::Next(f)
    }
}

impl Into<Attribute> for Routine {
    fn into(self) -> Attribute {
        Attribute::Functions(self)
    }
}