
use std::collections::BTreeSet;
use specs::Component as SpecsComponent;
use specs::{Entities, Join, ReadStorage, System, VecStorage};

use crate::{
    prelude::{NodeInterior, NodeResource, NodeVisitor, State},
    store::Store,
};

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

#[derive(Clone)]
struct EditorNode {
    config: Store<&'static str>,
}

impl Default for EditorNode {
    fn default() -> Self {
        Self {
            config: Store::default()
                .node("component")
                .node("input")
                .node("output"),
        }
    }
}

impl<'a> NodeVisitor<'a> for EditorNode {
    type Parameters = NodeComponent;

    fn call(&self, params: Self::Parameters) -> Self {
        let mut config = self.config.clone();
        match params {
            NodeComponent::Input(..) => config = config.link("component", "input"),
            NodeComponent::Output(..) => config = config.link("component", "output"),
        };

        Self {
            config
        }
    }

    fn evaluate(&self) -> Option<State> {
        todo!()
    }
}

#[derive(Clone)]
enum NodeComponent {
    Input(String),
    Output(String),
}

impl SpecsComponent for NodeComponent {
    type Storage = VecStorage<Self>;
}

impl<'a> System<'a> for EditorNode {
    type SystemData = (Entities<'a>, ReadStorage<'a, NodeComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let mut seen: BTreeSet<&'static str> = BTreeSet::new();
        let mut visited: BTreeSet<(Option<&'static str>, Option<&'static str>)> = BTreeSet::new();

        // loop through all components
        for (e, component) in (&data.0, &data.1).join() {
            let state = self.clone().call(component.clone());
            seen.clear();
            visited.clear();

            state.config.walk_ordered("component", &mut seen, &mut visited);

            if !visited.insert((Some("component"), Some("input"))) {
                // todo
            }

            if !visited.insert((Some("component"), Some("output"))) {
                // todo
            }
        }
    }
}
