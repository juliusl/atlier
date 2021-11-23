use std::{collections::BTreeMap, hash::{Hash, Hasher}};

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

pub trait Reducer {
    // Implementation returns the parameter they expect
    fn param_name() -> &'static str;

    // Implementation reduces an attribute 
    fn reduce(attribute: Option<AttributeValue>) -> Option<AttributeValue>;

    // This will be called by the runtime in order to decide whether or not reduce should be called again
    fn map(state: &BTreeMap<String, AttributeValue>) -> (u64, Option<AttributeValue>) {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        if let Some(v) = state.get(Self::param_name()) {
            v.hash(&mut hasher);
            (hasher.finish(), Some(v.to_owned()))
        } else {
            (0, None)
        }
    }
}