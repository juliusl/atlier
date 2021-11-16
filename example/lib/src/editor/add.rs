use crate::*;

impl<'a> Renderer<'a, Data, Editor> for AddEditorRenderer {
    type Artifact = AddNode<Editor>;

    fn render(&mut self, content: &ContentStore<Editor>, data: &Data, artifact: &Self::Artifact) {
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

impl<'a> Updater<'a, Data, Editor> for AddEditorUpdater {
    type Artifact = AddNode<Editor>;

    fn update(&mut self, content: &mut ContentStore<Editor>, data: &mut Data, artifact: &mut Self::Artifact) {
        match data.clone() {
            Data::Add(mut s) => {
                for i in artifact.get_outputs().elems.iter() {
                    let data = if let Some(EditorData::Add(a)) = content.get(i.content_id) {
                        Some(a)
                    } else {
                        let n = i.name.0.clone();
                        match (n.eq("sum"), n.eq("display")) {
                            (true, _) => Some(AddOutput::Sum(None)),
                            (_, true) => Some(AddOutput::Display(None)),
                            _ => None,
                        }
                    };

                    let data = if let Some(f) = data {
                        EditorData::Add(s.transition(f))
                    } else {
                        EditorData::Empty
                    };

                    content.set(i.content_id, &data);
                }
            }
            _ => {}
        }
    }
}
