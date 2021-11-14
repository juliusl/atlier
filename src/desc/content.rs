use crate::{Graph, desc::*};

pub struct ContentStore<N, V> 
where
    N: Node, 
    V: Debug + Default + Clone + Any
{
    typeid: std::any::TypeId,
    items: HashMap<ContentId, V>,
    references: HashMap<NodeId<N>, ContentId>,
}

impl<N, V> Default for ContentStore<N, V> 
where
    N: Node, 
    V: Debug + Default + Clone + Any
{
    fn default() -> Self {
        ContentStore::<N, V> {
            typeid: V::default().type_id(),
            items: HashMap::new(),
            references: HashMap::new()
        }
    }
}

impl<N, V> Listener for ContentStore<N, V>
where
    N: Node, 
    V: Debug + Default + Clone + Any,
{
    type Value = V;

    fn listen(&self, update: ContentUpdate<Self::Value>) {
        println!("{:?}", update)
    }
}

impl<N, V> Store for ContentStore<N, V>
where
    Self: Listener<Value = V>,
    N: Node, 
    V: Debug + Default + Clone + Any,
{
    type N = N;
    type Value = V; 

    fn get(&self, id: ContentId) -> Option<Self::Value> {
        if let Some(v) = self.items.get(&id) {
            return Some(v.clone())
        } else {
            None
        }
    }

    fn set(&mut self, content_id: ContentId, v: &Self::Value) {
        if v.type_id() == self.typeid {
            if let Some(prev) = self.items.insert(content_id, v.clone()) {
                let update = ContentUpdate::<Self::Value> { 
                    content_id: content_id, 
                    typeid: self.typeid, 
                    pre: prev, 
                    update: v.clone() };


                self.listen(update);
            }
        }
    }
}

impl<N, V> Graph for ContentStore<N, V> 
where
    N: Node, 
    V: Debug + Default + Clone + Any,
    {
        type N = N;
        type Link = V;

        fn get_links(&self, id: NodeId<Self::N>) -> Vec<Self::Link> {
            todo!()
        }
    }