use crate::{desc::*};

#[derive(Debug)]
pub struct ContentStore<N>
where
    N: Node,
{
    items: HashMap<ContentId, N::V>,
}

impl<N> Default for ContentStore<N>
where
    N: Node,
{
    fn default() -> Self {
        ContentStore::<N> {
            items: HashMap::new(),
        }
    }
}

impl<N> Listener for ContentStore<N>
where
    N: Node,
{
    type Value = N::V;

    fn listen(&self, update: ContentUpdate<Self::Value>) {
        println!("{:?}", update)
    }
}

impl<N> Store<N> for ContentStore<N>
where
    Self: Listener<Value = N::V>,
    N: Node,
{
    fn get(&self, id: ContentId) -> Option<N::V> {
        if let Some(v) = self.items.get(&id) {
            return Some(v.clone());
        } else {
            None
        }
    }

    fn set(&mut self, content_id: ContentId, v: &N::V) {
        if let Some(prev) = self.items.insert(content_id, v.clone()) {
            let update = ContentUpdate::<N::V> {
                content_id: content_id,
                pre: prev,
                update: v.clone(),
            };

            self.listen(update);
        } else {
            let init = ContentUpdate::<N::V> {
                content_id: content_id,
                pre: N::V::default(),
                update: v.clone(),
            };

            self.listen(init);
        }
    }
}
