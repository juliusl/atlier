use std::marker::PhantomData;
use crate::desc::*;
use crate::desc::content::*;
use crate::Node;

pub trait ArtifactContent {
    type Type;
    type Owner: Node;
    type Content: Default + Clone + 'static + Debug;

    fn new(id: Self::Type, name: String, content_id: ContentId) -> Self;
}

#[derive(Debug)]
pub enum ArtifactId<N> 
where 
    N: Node
{
    Node(N::NodeId),
    Attribute(N::AttributeId),
    Input(N::InputId),
    Output(N::OutputId)
}

#[derive(Debug)]
pub struct Artifact<N, C>
where
    N: Node,
{
    pub id: ArtifactId<N>,
    pub name: Name,
    pub type_id: std::any::TypeId,
    pub content_id: ContentId,
    phantom: PhantomData<C>,
}

#[derive(Debug)]
pub struct ArtifactCollection<N, C> 
where
    N: Node,
{
    pub elems: Vec<Artifact<N, C>>,
}

impl<N, C> Descriptor for Artifact<N, C>
where
N: Node,
C: Default + Clone + Debug + Any,
{
    type Store = ContentStore<N, C>;
    type Value = C;

    fn name(&self) -> Name {
        self.name.clone()
    }

    fn content(&self, store: &Self::Store) -> Option<C> {
        store.get(self.content_id)
    }
}

impl<N, C> ArtifactContent for Artifact<N, C>
where 
    N: Node,
    C: Default + Clone + 'static + Debug,
{
    type Type = ArtifactId<N>;
    type Owner = N;
    type Content = C;

    fn new(id: Self::Type, name: String, content_id: crate::ContentId) -> Self {
        Artifact::<N, C> {
                id: id,
                type_id: C::default().type_id(),
                content_id: content_id,
                name: crate::desc::Name::from(name),
                phantom: PhantomData::default(),
            }
    }
}

impl <N, C> Artifact<N, C> 
where 
    N: Node,
    C: Default + Clone + 'static + Debug,
{
    pub fn new_input(id: N::InputId, name: String, content_id: crate::ContentId) -> Self {
        Artifact::<N, C>::new(ArtifactId::<N>::Input(id), name, content_id)
    }

    pub fn new_output( id: N::OutputId, name: String, content_id: crate::ContentId) -> Self {

        Artifact::<N, C>::new(ArtifactId::<N>::Output(id), name, content_id)
    }

    pub fn new_attribute(id: N::AttributeId, name: String, content_id: crate::ContentId) -> Self {
        Artifact::<N, C>::new(ArtifactId::<N>::Attribute(id), name, content_id)
    }
}

