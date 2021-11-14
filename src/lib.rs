pub mod desc;

pub mod prelude;
use prelude::*; 

pub use derive::*;

use std::{any::Any, fmt::Debug};
use std::hash::Hash;

// A transition specifies that the implementing data has an invocation that has an output
// Once the data's parameters have been satisfied
// This leaves it up to the user to define the outputs for the data, without needing to worry about
// how the data is going to flow
pub trait Transition : Debug + Default + Clone + Any {
    type Output;

    // If the data has an output value, transition will return it,
    // As named this can have side effects on the caller, hence &mut 
    fn transition(&mut self, select_output: Self::Output) -> Self::Output;
}

// A node is anything that can be addressed, and in multi-tiered
// software, there are usually many components that need their own representation.

// Hence, implementing this trait allows you to control how,
// parts of the node are addressed on a more granular level.

// At a higher level, this allows the implementation to maintain
// a set of relationships opaque to the components. Meaning applicances,
// written on one set of hardware, should be able to be addressed in another set of hardware.

// This is analagous to the imaginary part of imaginary math, which allows for transforms to a higher
// dimensional space in order to simplify the operations needed to resolve the expression, 
// and then transforming the result back into it's source domain.

pub trait Node: Hash + PartialEq + Eq {
    type NodeId: Clone + Debug + Hash + PartialEq + Eq;
    type InputId: Clone + Debug + Hash + PartialEq + Eq;
    type OutputId: Clone + Debug + Hash + PartialEq + Eq;
    type AttributeId: Clone + Debug + Hash + PartialEq + Eq;
    type Data: Debug + Default + Clone + Any;

    fn next_node_id(&mut self) -> Self::NodeId;
    fn next_input_id(&mut self) -> Self::InputId;
    fn next_output_id(&mut self) -> Self::OutputId;
    fn next_attribute_id(&mut self) -> Self::AttributeId;
}

// When implemented, this will be called by the renderer system
// This allows client code to swap out rendering backends
pub trait Renderer : Node {
    type Artifact: State<N = Self>;

    fn render(&self, artifact: &Self::Artifact);
}

pub trait Graph {
    type N: Node;
    type Link;

    fn get_links(&self, id: NodeId<Self::N>) -> Vec<Self::Link>;
}

// This trait gets generated for nodes if they are used as entities 
pub trait State {
    type N: Node + Hash + Eq + PartialEq + Sync;
    type V: Default + Debug + Clone + Any;
    type Inputs;
    type Outputs;
    type Attributes;

    fn get_nodeid(&self) -> NodeId<Self::N>;
    fn get_inputs(&self) -> Self::Inputs;
    fn get_outputs(&self) -> Self::Outputs;
    fn get_attributes(&self) -> Self::Attributes;
}
