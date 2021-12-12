use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::{Hash, Hasher}
};

/// `Store` is a graph of nodes that store a value of `T` and the links to `T`
#[derive(Clone, Debug)]
pub struct Store<T>
where
    T: Hash + Clone + Into<T>,
{
    nodes: HashMap<u64, (T, HashSet<u64>)>,
}

impl<T> Hash for Store<T> 
where
T: Hash + Clone + PartialOrd,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nodes.iter().for_each(|e| {
            let (key, (value, ..)) = e; 
            {
                key.hash(state);
                value.hash(state); 
            }
        });
    }
}

impl<T> Default for Store<T>
where
T: Hash + Clone,
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

    /// `references` returns all nodes whose references contain `val`. 
    ///  If no nodes reference `val` returns `None`.
    pub fn references(&self, val: T) -> Option<Vec<T>> {
        let code = Self::get_hash_code(val);

        let references: Vec<T> = self.nodes.iter().filter_map(|f|{
            let (id, (val, references)) = f;

            let is_ref = references.iter().any(|f| {
                f ^ code == *id
            });

            if is_ref {
                Some(val.to_owned())
            } else {
                None
            }
        }).collect();

        if references.len() > 0 {
            Some(references)
        } else {
            None
        }
    }

    /// `edge_edge` creates a link between two edge nodes
    pub fn edge_edge<E>(&self, from: E, to: E) -> Self 
    where
        E: Into<T> + Clone
    {
        let f: T = from.clone().into();
        let t: T = to.clone().into();
        self
            .node(f.clone())
            .node(t.clone())
            .link(f.clone(), t.clone())
    }

    /// `edge_node` adds a node that rests at the edge of the graph data
    /// might be useful as an entrypoint from external data into graph data
    pub fn edge_node<E>(&self, val: E) -> Self 
    where
        E: Into<T>
    {
        self.node(val.into())
    }

    /// `edge_link` creates a link from an edge node to a store node
    pub fn edge_link<E>(&self, from: E, to: T) -> Self 
    where
        E: Into<T>
    {
        self.link(from.into(), to)
    }

    /// `edge_walk_ordered` walks the graph starting from an edge node in order
    pub fn edge_walk_ordered<E>(&self, from: E) ->  (BTreeSet<T>, BTreeSet<(Option<T>, Option<T>)>) 
    where
        E: Into<T>,
        T: Ord
    {
        let mut seen: BTreeSet<T> = BTreeSet::new();
        let mut visited: BTreeSet<(Option<T>, Option<T>)> = BTreeSet::new();

        self.walk_ordered(from.into(),&mut seen, &mut visited);

        (seen, visited)
    }

    /// `edge_walk` walks the graph starting from an edge node 
    pub fn edge_walk<E>(&self, from: E) ->  (HashSet<T>, HashSet<(Option<T>, Option<T>)>) 
    where
        E: Into<T>,
        T: Eq
    {
        let mut seen: HashSet<T> = HashSet::new();
        let mut visited: HashSet<(Option<T>, Option<T>)> = HashSet::new();

        self.walk(from.into(),&mut seen, &mut visited);

        (seen, visited)
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

struct IndirectIndexer();

impl Into<Indexer> for IndirectIndexer {
    fn into(self) -> Indexer {
        Indexer("test".to_string(), 0)
    }
}

#[derive(Default, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
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

    let indexer = indexer
        .node(Indexer("test".to_string(), 5))
        .edge_node(IndirectIndexer());

    let indexer = indexer
    .link(
        Indexer("test".to_string(), 5),
        Indexer("test".to_string(), 5),
    )
    .edge_link(
        IndirectIndexer(),
        Indexer("test".to_string(), 5), 
    );

    let (seen_ordered, visited_ordered) = indexer.edge_walk_ordered(IndirectIndexer());

    assert!(seen_ordered.contains(&IndirectIndexer().into()));
    assert!(visited_ordered.contains(&(Some(IndirectIndexer().into()), Some(Indexer("test".to_string(), 5)))));
}
