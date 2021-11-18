use specs::DenseVecStorage;
use specs::prelude::*;
use super::{NodeResource, resource::EditorResource};

fn expression(name: &'static str) -> Vec<NodeResource> {
    vec![
        NodeResource::Title(name),
        NodeResource::Input(||"lhs", None),
        NodeResource::Input(||"rhs", None),
        NodeResource::Output(||"output", None),
    ]
}

pub struct Sum(EditorResource); 

impl Default for Sum {
    fn default() -> Self {
        Sum(EditorResource::Node{
            resources: expression("sum"),
            id: None
        })
    }
}

impl Into<EditorResource> for Sum {
    fn into(self) -> EditorResource {
        self.0
    }
}

impl Component for Sum {
    type Storage = DenseVecStorage<Self>; 
}
