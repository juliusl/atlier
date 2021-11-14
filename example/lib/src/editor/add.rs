use crate::*;

// TODO: Implemented by lib owner
impl Renderer for Editor {
    type Artifact = AddNode<Self, Data>;

    fn render(&self, artifact: &Self::Artifact) {
        println!("{:#?}", artifact.get_nodeid());
        for i in artifact.get_attributes().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id)
        }

        for i in artifact.get_inputs().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id)
        }

        for i in artifact.get_outputs().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id)
        }
    }
}
