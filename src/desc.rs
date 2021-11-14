pub mod artifact;
pub mod content;

use crate::Node;
use std::any::{Any};
use std::collections::HashMap;
use std::fmt::Debug;
use core::hash::Hash;

// This is a common component many entities have
#[derive(Debug, Clone)]
pub struct Name(pub String);

// This is a common id used for storage
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ContentId 
{
    pub id: i32,
    pub typeid: std::any::TypeId,
}

impl From<String> for Name {
    fn from(n: String) -> Self {
        Name(n)
    }
}

// This is a common component all entities should have
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NodeId<N>
where
    N: Node,
{
    pub id: N::NodeId,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ContentUpdate<V> 
where 
    V: Default + Debug + Clone + Any
{
    content_id: ContentId,
    typeid: std::any::TypeId,
    pre: V,
    update: V,
}

trait Descriptor {
    type Store: Store; 
    type Value: Default + Debug + Clone + Any;
    
    fn name(&self) -> Name;
    fn content(&self, store: &Self::Store) -> Option<Self::Value>;
}

trait Listener {
    type Value: Default + Debug + Clone + Any;

    fn listen(&self, update: ContentUpdate<Self::Value>);
}

trait Store {
    type N: Node;
    type Value: Default + Debug + Clone + Any;

    fn get(&self, id: ContentId) -> Option<Self::Value>;
    fn set(&mut self, id: ContentId, v: &Self::Value);
}

