use crate::{store::Store, prelude::Attribute};

use super::{CoreSystem, State};

mod node;
pub use node::FSNode;

#[derive(Clone)]
pub struct Filesystem 
{
    store: Store<FSNode>
}

impl Default for Filesystem {
    fn default() -> Self {
        Self { store: Store::<FSNode>::default()
            .node(FSNode::Root)
            .node(FSNode::default())
            .link(FSNode::Root, FSNode::default())
        }
    }
}

impl Into<FSNode> for Filesystem {
    fn into(self) -> FSNode {
        FSNode::Volume(".default")
    }
}

impl CoreSystem for Filesystem {
    type Node = FSNode;

    fn get_store(&mut self) -> Store<Self::Node> {
        self.store.clone()
    }

    fn update(&mut self, next: Store<Self::Node>) -> Self {
        Self {
            store: next
        }
    }
}

#[test]
fn test_filesystem_tooling() {
    let state = State::default();
    let mut fs = Filesystem::default();

    let fs = &mut fs.install_tooling(state.clone());

    let (_, visited) = fs.get_store().edge_walk(state.clone());

    assert!(visited.clone().contains(&(
        Some(State::default().into()), 
        Some(Filesystem::default().into()))
    ));

    assert!(!visited.clone().contains(&(
        Some(State::default()
            .insert("test", Attribute::from(0.00))
            .into()), 
        Some(Filesystem::default().into())
    )));

    for entry in visited {
        println!("{:?}", entry);
    }

    if let Some(refs) = fs.get_store().references(FSNode::Root) {
        refs.iter().for_each(|f| println!("{:?}", f));
    } else {
        println!("no references")
    }
}