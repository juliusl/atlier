use crate::*;

// TODO: Implemented by lib owner
impl<'a> Renderer<'a, Data> for Editor {
    type Artifact = AddNode<Editor>;

    fn render(&self, content: &ContentStore<Editor>, data: &Data, artifact: &Self::Artifact) {
        println!("{:?}", data);
        println!("{:#?}", artifact.get_nodeid());
        for i in artifact.get_attributes().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id);

            let c = i.content(content);
            println!("{:?}", c);
        }

        for i in artifact.get_inputs().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id);

            let c = i.content(content);
            println!("{:?}", c);
        }

        for i in artifact.get_outputs().elems.iter() {
            println!("{:#?}: {:#?}, {:#?}", i.id, i.name.clone(), i.content_id);

            let c = i.content(content);
            println!("{:?}", c);
        }
    }
}
