use std::{collections::BTreeMap,};

use specs::{Entities, System};

use super::{EditorResource, NodeExterior, NodeInterior, NodeResource, NodeVisitor, Reducer, Routines, State, Value};

#[derive(Debug, Clone, Hash)]
pub enum Attribute {
    Literal(Value),
    Functions(Routines),
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

    fn display(label: String, width: f32, ui: &imgui::Ui, value: &mut Attribute) {
        Self::input(label, width, ui, value)
    }
}

impl<'a> NodeVisitor<'a> for Attribute {
    type Parameters = &'static str; 

    fn evaluate(&self) -> Option<State> {
        let state: State = self.clone().into();
        Some(state.into())
    }

    fn call(&self, name: Self::Parameters) -> Self {
        todo!()
    }
}

impl<'a> NodeInterior<'a> for Attribute {
    type Visitor = Self;
}

impl<'a> System<'a> for Attribute {
    type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}

impl Into<State> for Attribute {
    fn into(self) -> State {
        let state = State::default();
        let state = match &self {
            Attribute::Literal(literal) => {
            match literal {
                Value::Float(_) => state.insert("float", self),
                Value::Int(_) => state.insert("int", self),
                Value::Bool(_) => state.insert("bool", self),
                Value::FloatRange(_, _, _) => state.insert("float_range", self),
                Value::IntRange(_, _, _) => state.insert("int_range", self),
                Value::TextBuffer(_) => state.insert("text_buffer", self),
            }
        },
            Attribute::Functions(routines) => {
                match routines {
                Routines::Name(_) => state.insert("name", self),
                Routines::Select(_) => state.insert("select", self),
                Routines::Reduce(_) => state.insert("reduce", self),
                Routines::Transform(_) => state.insert("transform", self),
                Routines::Next(_) => state.insert("next", self),
            }},
            Attribute::Map(_) => { 
                state.insert("map", self) },
            Attribute::Error(_) => {
                state.insert("error", self)},
            Attribute::Empty => state,
        };

        state
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

impl From<Routines> for Attribute {
    fn from(r: Routines) -> Self {
        Attribute::Functions(r)
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
            Attribute::Functions(_) => todo!(),
        }
    }
}
