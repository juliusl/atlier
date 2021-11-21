use std::collections::BTreeMap;

use super::{AttributeValue};

pub trait  NodeVisitor {

    fn evaluate(&self, state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue>;    
}