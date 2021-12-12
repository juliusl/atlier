
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FSNode {
    Root,
    Volume(&'static str),
    Mount(String),
}

impl Default for FSNode {
    fn default() -> Self {
        FSNode::Volume(".default")
    }
}