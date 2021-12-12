
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FSNode {
    Root,
    Volume(&'static str),
    Mount(&'static str, u64),
    Record(&'static str, &'static str),
    File(&'static str),
}

impl Default for FSNode {
    fn default() -> Self {
        FSNode::Volume(".default")
    }
}