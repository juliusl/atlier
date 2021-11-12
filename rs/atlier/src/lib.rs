
pub use derive::*;

// A transition specifies that the implementing data has an invocation that has an output
// Once the data's parameters have been satisfied
// This leaves it up to the user to define the outputs for the data, without needing to worry about
// how the data is going to flow
pub trait Transition {
    type Data;
    type Output;

    // If the data has an output value, transition will return it
    fn transition(data: Self::Data, select_output: Self::Output) -> Self::Output;
}


// A node is a collection of id(s)/coordinates
pub trait Node {
    type InputId;
    type OutputId;
    type AttributeId;
}
