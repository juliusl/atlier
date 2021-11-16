use crate::desc::*;
use crate::desc::content::*;
use crate::Node;

pub trait ArtifactContent {
    type Type: IdType;

    fn new(id: Self::Type, name: String, typeid: std::any::TypeId) -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ArtifactId<N> 
where 
    N: Node
{
    Node(N::NodeId),
    Attribute(N::AttributeId),
    Input(N::InputId),
    Output(N::OutputId)
}

impl<N> IdType for ArtifactId<N>
where
    N: Node
{
    type Id = Self;
}

impl<N> BitXor<u64> for ArtifactId<N> 
where 
    N: Node
{
    type Output = u64;

    fn bitxor(self, rhs: u64) -> u64 {
        match self {
            ArtifactId::<N>::Node(n) => n.into() ^ rhs,
            ArtifactId::<N>::Attribute(a) => a.into() ^ rhs,
            ArtifactId::<N>::Input(i) => i.into() ^ rhs,
            ArtifactId::<N>::Output(o) => o.into() ^ rhs,
        }
    }
}

impl<N> Into<u64> for ArtifactId<N> 
where 
    N: Node
{
    fn into(self) -> u64 {
        match self {
            ArtifactId::<N>::Node(n) => n.into(),
            ArtifactId::<N>::Attribute(a) => a.into(),
            ArtifactId::<N>::Input(i) => i.into(),
            ArtifactId::<N>::Output(o) => o.into(),
        }
    }
}

#[derive(Debug)]
pub struct Artifact<N>
where
    N: Node,
{
    pub id: ArtifactId<N>,
    pub name: Name,
    pub type_id: std::any::TypeId,
    pub content_id: ContentId,
}

#[derive(Debug)]
pub struct ArtifactCollection<N> 
where
    N: Node,
{
    pub elems: Vec<Artifact<N>>
}

impl<N> Descriptor for Artifact<N>
where
N: Node,
{
    type Node = N;
    type Store = ContentStore<N>;
    type Value = N::V;

    fn name(&self) -> Name {
        self.name.clone()
    }

    fn content(&self, store: &Self::Store) -> Option<Self::Value> {
        store.get(self.content_id)
    }
}

impl<N> ArtifactContent for Artifact<N>
where 
    N: Node,
{
    type Type = ArtifactId<N>;

    fn new(id: Self::Type, name: String, typeid: std::any::TypeId) -> Self {
        Artifact::<N> {
                id: id.clone(),
                type_id: typeid,
                content_id: ContentId::new::<Self::Type>(id.clone(), typeid),
                name: crate::desc::Name::from(name),
            }
    }
}

impl <N> Artifact<N> 
where 
    N: Node,
{
    pub fn new_input(id: N::InputId, name: String, typeid: std::any::TypeId) -> Self {
        Artifact::<N>::new(ArtifactId::<N>::Input(id), name, typeid)
    }

    pub fn new_output( id: N::OutputId, name: String, typeid: std::any::TypeId) -> Self {

        Artifact::<N>::new(ArtifactId::<N>::Output(id), name, typeid)
    }

    pub fn new_attribute(id: N::AttributeId, name: String, typeid: std::any::TypeId) -> Self {
        Artifact::<N>::new(ArtifactId::<N>::Attribute(id), name, typeid)
    }
}
