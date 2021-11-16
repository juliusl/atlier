pub mod artifact;
pub mod content;

use crate::Node;
use std::collections::hash_map::DefaultHasher;
use std::any::{Any};
use std::collections::HashMap;
use std::fmt::Debug;
use core::hash::Hash;
use std::hash::Hasher;
use std::ops::BitXor;

// This is a common component many entities have
#[derive(Debug, Clone)]
pub struct Name(pub String);

pub trait IdType
{
    type Id: Clone + Debug + Hash + PartialEq + Eq + Into<u64> + BitXor<u64>;
}

// This is a common id used for storage
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ContentId
{
    pub id: u64,
    pub typeid: std::any::TypeId,
    pub hash: u64,
}

impl ContentId {
    pub fn new<ID>(id: ID, typeid: std::any::TypeId) -> Self 
    where
        ID: Hash + Into<u64>
    {
        let mut hasher = DefaultHasher::default();
        typeid.hash(&mut hasher);
        id.hash(&mut hasher);

        ContentId {
            id: id.into(),
            hash: hasher.finish(),
            typeid: typeid,
        }
    }
}

impl IdType for ContentId {
    type Id = Self;
}

impl Into<u64> for ContentId {
    fn into(self) -> u64 {
        self.hash
    }
}

impl BitXor<u64> for ContentId {
    type Output = u64;

    fn bitxor(self, rhs: u64) -> Self::Output {
        self.hash ^ rhs
    }
}

impl From<String> for Name {
    fn from(n: String) -> Self {
        Name(n)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ContentUpdate<V> 
where 
    V: Default + Debug + Clone + Any
{
    content_id: ContentId,
    pre: V,
    update: V,
}

pub trait Descriptor {
    type Node: Node; 
    type Store: Store<Self::Node>; 
    type Value: Default + Debug + Clone + Any;
    
    fn name(&self) -> Name;
    fn content(&self, store: &Self::Store) -> Option<Self::Value>;
}

pub trait Listener {
    type Value: Default + Debug + Clone + Any;

    fn listen(&self, update: ContentUpdate<Self::Value>);
}

pub trait Store<N>
where 
    N: Node 
{
    fn get(&self, id: ContentId) -> Option<N::V>;
    fn set(&mut self, id: ContentId, v: &N::V);
}

