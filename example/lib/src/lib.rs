
pub use atlier::prelude::*;
pub use specs::prelude::*;

mod editor;
pub use editor::*;
use std::fmt::Debug;
use std::any::Any;
use std::hash::Hash;


#[derive(Transition, Node, Clone, Debug)]
#[output(sum, i32)]
#[output(display, String)]
pub struct Add {
    lhs: i32,
    rhs: i32, 
}

impl Default for Add {
    fn default() -> Self {
        Add { lhs: 0, rhs: 1 }
    }
}

// TODO: Implemented by user
impl AddOutputs for Add {
    fn sum(lhs: i32, rhs: i32) -> Option<i32> {
        Some(lhs+rhs)
    }
    fn display(lhs: i32, rhs: i32) -> Option<String> {
        Some(format!("{} + {} = {}", lhs, rhs, lhs+rhs))
    }
}


#[derive(Clone, Debug)]
pub enum Data {
    Initial,
}

impl Default for Data {
    fn default() -> Self {
        Data::Initial
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Renderer)]
#[render(Add, Data)]
pub struct Editor {
    id: i32
}

impl Editor {
    fn next_id(&mut self) -> i32 {
        let next = self.id;

        self.id = next+1; 

        next
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            id: 0
        }
    }
}

#[derive(Debug, Clone)]
pub enum EditorData {
    Labels, 
    Empty,
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData::Empty
    }
}

impl Node for Editor {
    type NodeId = i32;
    type InputId = i32;
    type OutputId = i32;
    type AttributeId = i32;
    type Data = EditorData;

    fn next_node_id(&mut self) -> Self::NodeId {
        self.next_id()
    }

    fn next_input_id(&mut self) -> Self::InputId {
        self.next_id()
    }

    fn next_output_id(&mut self) -> Self::OutputId {
        self.next_id()
    }

    fn next_attribute_id(&mut self) -> Self::AttributeId {
        self.next_id()
    }
}

