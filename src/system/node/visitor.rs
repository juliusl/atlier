use std::collections::BTreeMap;

use super::{AttributeValue, EditorResource};

// These are traits to help define different types of nodes that can be used from the editor 

pub trait NodeInterior<'a> {
    type Literal;
    type Visitor: NodeVisitor + From<Self::Literal>;

    // Accept a visitor to convert the current interior state
    fn accept(state: &'a BTreeMap<String, AttributeValue>) -> Self::Visitor;
}

pub trait NodeVisitor {
    // Evaluate a result from this visitor
    fn evaluate(&self) -> Option<AttributeValue>;
}

pub trait NodeExterior {
    // Return an editor resource to represent the exterior of the node
    fn resource(nodeid: Option<imnodes::NodeId>) -> EditorResource;
}
