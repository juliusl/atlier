use imnodes::IdentifierGenerator;
use specs::{System, Entities, Write, Component, DenseVecStorage, VecStorage, WriteStorage, Join};

use crate::system::State;

use super::{EditorResource, NodeVisitor, NodeInterior};

#[derive(Clone)]
pub struct Module {
    resources: Vec<EditorResource>,
}

impl Component for Module {
    type Storage = VecStorage<Self>;
}

impl<'a> System<'a> for Module {
    type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {

        let module = 
            State::default()
            .visit::<Initializer>()
            .returns("lhs");

        todo!()
    }
}

impl<'a> NodeVisitor<'a> for Module {
    type Parameters = EditorResource;

    fn call(&self, resource: Self::Parameters) -> Self {
        let Module { mut resources } = self.clone(); 

        resources.push(resource); 

         Module { resources: resources }
    }

    fn evaluate(&self) -> Option<State> {
        todo!()
    }
}

impl From<State> for Module {
    fn from(_: State) -> Self {
        todo!()
    }
}      
pub struct Initializer {
    idgen: IdentifierGenerator
}

impl<'a> NodeInterior<'a> for Initializer {
    type Visitor = Module;
}

impl<'a> System<'a> for Initializer {
    type SystemData = WriteStorage<'a, Module>;

    fn run(&mut self, data: Self::SystemData) {
        for module in data.join() {
            for ed in &module.resources { 
                if let EditorResource::Node { id: None, resources } = ed {
                    EditorResource::Node { 
                        id: Some(self.idgen.next_node()), 
                        resources: resources.clone() 
                    };
                }
            }
        }

        todo!()
    }
}