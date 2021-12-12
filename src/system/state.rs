use std::{collections::BTreeMap, hash::{Hash, Hasher}};

use super::{Attribute, NodeVisitor, Routine, Reducer, NodeInterior, Value, filesystem::FSNode};

#[derive(Clone, Hash)]
pub struct State(BTreeMap<String, Attribute>, Option<Vec<Update>>);

#[derive(Clone, Hash)]
pub enum Update {
    Insert(String, Attribute),
    Delete(String),
    Merge(BTreeMap<String, Attribute>),
    SelectAttribute(String),
    Reduce(),
}

impl Default for State {
    fn default() -> Self {
        Self(BTreeMap::default(), None)
    }
}

impl Into<FSNode> for State {
    fn into(self) -> FSNode {
        let hash_code = self.get_hash_code();

        FSNode::Mount(format!("state_{}", hash_code))
    }
}

impl<'a> NodeVisitor<'a> for State {
    type Parameters = Update;

    fn dispatch(&self, params: Self::Parameters) -> Self {
        match &self {
            State(state, None) => {
                let updates = vec![params]; 

                State(state.clone(), Some(updates))
            }
            State(state, Some(updates)) => {
                let mut updates = updates.clone();
                updates.push(params); 

                State(state.clone(), Some(updates))
            }
        }
    }

    fn evaluate(&self) -> Option<State> {
        match &self {
            State(state, Some(updates)) => {
                let mut next = state.clone();
                updates.iter().for_each(|u| {
                    match u {
                        Update::Insert(key, value) => {
                            next.insert(key.to_string(), value.clone());
                        }
                        Update::Delete(key) => {
                            next.remove(key);
                        }
                        Update::Merge(map) => {
                            for (key, value) in map {
                                next.insert(key.to_string(), value.to_owned());
                            }
                        }
                        Update::SelectAttribute(key) => {
                            next.insert("selected_attribute".to_string(), Attribute::Literal(Value::TextBuffer(key.clone())));
                        }
                        Update::Reduce() => {
                            next.insert("reducer".to_string(), Attribute::Literal(Value::Bool(true)));
                        }
                    }
                });

                // todo: This could be cleaned up a bit
                let next_state = State(next.clone(), None);
                let next_hash_code = &next_state.get_hash_code();

                match self.get("selected_attribute") {
                    Some(Attribute::Literal(Value::TextBuffer(key))) => {
                        match self.get(key.clone()) {
                            Some(Attribute::Functions(Routine::Select(select))) => {
                                if let (selected_hash_code, Some(value)) = select(next_state) {
                                    if selected_hash_code != *next_hash_code {
                                        Some(State(next, None).insert(&key.clone(), value))
                                    } else {
                                        Some(State(next, None))
                                    }
                                } else {
                                    Some(State(next, None))
                                }
                            }
                            _ => {
                                Some(State(next, None))
                            }
                        }
                    }
                    _ => {
                        Some(State(next, None))
                    }
                }
            }
            _ => None,
        }
    }
}

impl State {
    /// `get` returns the latest version of the attribute
    /// `get` will flatten all messages into a state before getting the next value. This has no side effects on the original collection.
    pub fn get<T>(&self, key: T) -> Option<Attribute> 
    where
        T: ToString
    {
        if let Some(State(map, ..)) = self.clone().evaluate() {
            match map.get(&key.to_string()) {
                Some(attr) => Some(attr.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// `get_hash_code` returns the current hash value for this map
    pub fn get_hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();

        self.hash(&mut hasher);

        hasher.finish()
    }

    /// `snapshot` returns a clone of state as-is w/o updates
    pub fn snapshot(&self) -> Self {
        State(self.0.clone(), None)
    }

    /// `insert` is a helper method to dispatch an insert update
    pub fn insert<V>(&self, key: &str, value: V) -> Self
    where
        V: Into<Attribute>,
    {
        self.dispatch(Update::Insert(key.to_string(), value.into()))
    }

    /// `merge` is a helper method to dispatch a merge update
    pub fn merge<M>(&self, map: M) -> Self
    where
        M: Into<BTreeMap<String, Attribute>>,
    {
        self.dispatch(Update::Merge(map.into()))
    }

    /// `delete` is a helper method to dispatch a delete update
    pub fn delete(&self, key: &str) -> Self {
        self.dispatch(Update::Delete(key.to_string()))
    }

    /// `map` creates a clone of a subset of parameters from `State`
    pub fn map(&self, parameters: &[&'static str]) -> Self {
        let mut mapped = Self::default();
        parameters
            .iter()
            .map(|p| (p, self.get(p)))
            .filter_map(|p| match p {
                (name, Some(attr)) => Some((name, attr)),
                _ => None,
            })
            .for_each(|(n, a)| {
                mapped = mapped.insert(*n, a);
            });

        mapped
    }

    /// `select` inserts a Select routine at `key` and sets the selected_attribute value to `key`
    pub fn select(&self, key: &str, select: fn(State) -> (u64, Option<Attribute>)) -> Self {
        self.dispatch(Update::SelectAttribute(key.to_string()))
            .insert(key, Attribute::Functions(Routine::Select(select)))
    }

    /// `visit` allows a visitor to initialize from this state
    pub fn visit<'a, T>(&self) -> T::Visitor
    where
        T: NodeInterior<'a>,
    {
        T::accept(self.clone())
    }
}

impl Into<BTreeMap<String, Attribute>> for State {
    fn into(self) -> BTreeMap<String, Attribute> {
        if let Some(next) = self.clone().evaluate() {
            next.0
        } else {
            self.0
        }
    }
}

impl From<&BTreeMap<String, Attribute>> for State {
    fn from(state: &BTreeMap<String, Attribute>) -> Self {
        State(state.clone(), None)
    }
}

#[test]
fn test_dispatch() {
    let state = State::default();
    let old = state.get_hash_code();

    let state = state
        .insert("test", 10.0)
        .insert("test", 14.0)
        .evaluate()
        .unwrap();

    let new = state.get_hash_code();
    assert_ne!(old, new);

    if let Some(v) = state.get("test") {
        assert_eq!(14.0, v.to_owned().into());
    }

    let evaluated_state = state
        .merge(State::default().insert("test", 17.0).insert("test2", 18.0))
        .evaluate()
        .unwrap();

    let newest = evaluated_state.get_hash_code();
    assert_ne!(new, newest);

    if let Some(v) = state.get("test") {
        assert_eq!(17.0, v.to_owned().into());
    }

    let evaluated_state = state
        .merge(State::default().insert("test", 17.0).insert("test2", 18.0))
        .evaluate()
        .expect("was just created so it should be evaluated");

    let latest = evaluated_state.get_hash_code();
    assert_eq!(
        newest, latest,
        "The hash code should not change after an inert merge"
    );

    let state = state.delete("test2");
    assert!(state.get("test2").is_none())
}
