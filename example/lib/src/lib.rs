
pub use atlier::prelude::*;
pub use specs::prelude::*;

mod editor;
pub use editor::*;
use std::fmt::Debug;
use std::any::Any;
use std::hash::Hash;
use std::ops::Deref;


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
    Add(Add),
}

impl Default for Data {
    fn default() -> Self {
        Data::Initial
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Renderer)]
#[render(Add, Data)]
pub struct Editor {
    id: u64
}

impl Component for Data {
    type Storage = DenseVecStorage<Self>;
}

impl Editor {
    fn next_id(&mut self) -> u64 {
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
    Empty,
    Labels, 
    Integer(i32),
    Add(AddOutput),
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData::Empty
    }
}

impl Node for Editor {
    type NodeId = u64;
    type InputId = u64;
    type OutputId = u64;
    type AttributeId = u64;
    type K = ContentId; 
    type V = EditorData;
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

