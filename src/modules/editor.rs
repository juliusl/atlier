use specs::{System, Entities};

use crate::prelude::{NodeInterior, NodeVisitor, State, NodeResource};


struct Editor;

impl<'a> NodeInterior<'a> for Editor {
    type Visitor = Component;
}

#[derive(Clone)]
struct Component(State);

impl Component {
    fn input(&self, param: fn() -> &'static str) -> Self {

        self.call(NodeResource::Input(param, None))
    }
}

#[test]
fn test_xor_link() {
    let a: u64 = 10;
    let b: u64 = 5; 

    let link = a ^ b;

    assert_eq!(link^a, b);
    assert_eq!(link^b, a);
}

impl<'a> NodeVisitor<'a> for Component {
    type Parameters = NodeResource;

    fn call(&self, params: Self::Parameters) -> Self {
        todo!()
    }

    fn evaluate(&self) -> Option<crate::prelude::State> {
        todo!()
    }
}

impl From<State> for Component {
    fn from(_: State) -> Self {
        todo!()
    }
}

impl<'a> System<'a> for Component {
    type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {

        // Should this be the thing rendering?
        todo!()
    }
}
