use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::{Hash, Hasher},
};

/// Store is a graph of nodes that store a value of T and the links to T
#[derive(Clone)]
pub struct Store<T>
where
    T: Hash + Clone,
{
    nodes: HashMap<u64, (T, HashSet<u64>)>,
}

impl<T> Default for Store<T>
where
    T: Hash + Clone + Default,
{
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

/// This store maintains a graph structure where the value and links to the value are stored
/// The only thing required to use this store is an instance of the type being stored
/// Each entry in the graph stores the element and a HashSet of the links
/// Each entry is keyed to the hash_code evaluation of the value being stored
/// Links between nodes are stored as a XOR-Link where `link = from ^ to`
/// where `from` and `to` are keys to the node
/// In order to traverse, starting from element T with links L
/// first the hash_code of T is evaluated `hash_code(T)`
/// links are iterated and to compute the destination the xor is taken `for link in links; let destination_key = hash_code(T) ^ link;`
/// then the destination node can be retrieved via the destination_key
impl<T> Store<T>
where
    T: Hash + Clone,
{
    /// `node` adds a node to the underlying graph for `val`
    /// If val is already a node, then no changes are made
    pub fn node(&self, val: T) -> Self {
        let mut next = self.nodes.clone();

        let hash_code = Self::get_hash_code(val.clone());

        if !next.contains_key(&hash_code) {
            next.insert(hash_code, (val, HashSet::new()));
        }

        Self { nodes: next }
    }

    /// `link` creates a link between two nodes in the underlying graph
    pub fn link(&self, from: T, to: T) -> Self {
        let from_hash_code = Self::get_hash_code(from);
        let to_hash_code = Self::get_hash_code(to);

        let mut next = self.nodes.clone();
        let link_code = from_hash_code ^ to_hash_code;

        // Cycle
        if link_code == 0 {
            return Self { nodes: next };
        }

        // Values must actually exist, otherwise it's a no-op
        if let (Some((v, v_links)), Some((v2, v2_links))) = (
            self.nodes.get(&from_hash_code),
            self.nodes.get(&to_hash_code),
        ) {
            let mut v_links = v_links.clone();
            let mut v2_links = v2_links.clone();

            if v_links.insert(link_code) {
                next.insert(from_hash_code, (v.clone(), v_links));
            }

            if v2_links.insert(link_code) {
                next.insert(to_hash_code, (v2.to_owned(), v2_links));
            }
        }

        Self { nodes: next }
    }

    /// `get` returns the current state of the node of `val`
    pub fn get(&self, val: T) -> Option<&(T, HashSet<u64>)> {
        let h = Self::get_hash_code(val);

        self.nodes.get(&h)
    }

    /// `visit` returns all of the links to the node of `val`
    pub fn visit(&self, val: T) -> Vec<(Option<T>, Option<T>)> {
        let mut visited: Vec<(Option<T>, Option<T>)> = vec![];

        if let Some((from, links)) = self.get(val.clone()) {
            let from_code = Self::get_hash_code(val);
            links.iter().for_each(|link| {
                let to_code = link ^ from_code;

                if let Some((to, ..)) = self.nodes.get(&to_code) {
                    visited.push((Some(from.clone()), Some(to.clone())));
                }
            });
        }

        visited
    }

    /// Walks all paths starting from val
    pub fn walk(&self, val: T, seen: &mut HashSet<T>, visited: &mut HashSet<(Option<T>, Option<T>)>)
    where
        T: Eq,
    {
        let start = val.clone();

        if seen.insert(start) {
            self.visit(val).iter().for_each(|link| {
                match link {
                    (Some(from), Some(to)) => {
                        if visited.contains(&(Some(to.clone()), Some(from.clone()))) {
                            return;
                        }

                        if visited.insert((Some(from.clone()), Some(to.clone()))) {
                            self.walk(to.clone(), seen, visited)
                        }
                    }
                    _ => unreachable!("visit only returns links"),
                };
            });
        }
    }

    /// Walks all paths starting from val
    /// Uses a BTreeSet to sort the values as they are visited
    pub fn walk_ordered(
        &self,
        val: T,
        seen: &mut BTreeSet<T>,
        visited: &mut BTreeSet<(Option<T>, Option<T>)>,
    ) where
        T: Ord,
    {
        let start = val.clone();

        if seen.insert(start) {
            self.visit(val).iter().for_each(|link| {
                match link {
                    (Some(from), Some(to)) => {
                        if visited.contains(&(Some(to.clone()), Some(from.clone()))) {
                            return;
                        }

                        if visited.insert((Some(from.clone()), Some(to.clone()))) {
                            self.walk_ordered(to.clone(), seen, visited)
                        }
                    }
                    _ => unreachable!("visit only returns links"),
                };
            });
        }
    }

    fn get_hash_code(val: T) -> u64 {
        let mut hash_code = std::collections::hash_map::DefaultHasher::default();

        val.hash(&mut hash_code);

        hash_code.finish()
    }
}

#[test]
fn test_store() {
    let store = Store::<&'static str>::default();
    let store = store
        .node("test-node-1")
        .node("test-node-2")
        .node("test-node-3")
        .node("test-node-4");
    let store = store
        .link("test-node-1", "test-node-2")
        .link("test-node-2", "test-node-1")
        .link("test-node-2", "test-node-3")
        .link("test-node-4", "test-node-3");

    let visited = store.visit("test-node-1").iter().any(|v| match v {
        (Some("test-node-1"), Some("test-node-2")) => true,
        _ => false,
    });

    assert!(visited, "did not find the expected link");

    let mut seen: HashSet<&'static str> = HashSet::new();
    let mut visited: HashSet<(Option<&'static str>, Option<&'static str>)> = HashSet::new();

    store.walk("test-node-1", &mut seen, &mut visited);

    seen.iter().for_each(|v| println!("{:?}", v));
    visited.iter().for_each(|v| println!("{:?}", v));

    let mut seen_ordered: BTreeSet<&'static str> = BTreeSet::new();
    let mut visited_ordered: BTreeSet<(Option<&'static str>, Option<&'static str>)> =
        BTreeSet::new();

    store.walk_ordered("test-node-1", &mut seen_ordered, &mut visited_ordered);

    seen_ordered.iter().for_each(|v| println!("{:?}", v));
    visited_ordered.iter().for_each(|v| println!("{:?}", v));
}

#[derive(Default, Clone, Hash)]
struct Indexer(String, u64);

/// You can declare the links even if they don't exist yet..
/// And then when it does exist, the walk should work..
///
/// You can speculatively declare links on the store.
/// ```
/// let store = store
///     .link("component", "input")
///     .link("component", "output")
/// ```
/// elsewhere, the link can be checked for in O(1) after the store is walked.
/// Then, code can be executed if the the speculated link was seen.
/// ```
/// store.walk_ordered("component")
/// if seen.insert("input") {
///     let input_id = id_gen.next_input();
/// }
/// ```
/// while previously, elsewhere, the node can declare the actual configuration of nodes
/// ```
/// let mut store = store.node("component");
/// if (input) {
///   store = store.node("input");
/// }
/// ```

#[test]
fn test_indexer() {
    let indexer = Store::<Indexer>::default();

    indexer.node(Indexer("test".to_string(), 5));

    indexer.link(
        Indexer("test".to_string(), 5),
        Indexer("test".to_string(), 5),
    );
}
