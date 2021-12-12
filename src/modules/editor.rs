
use std::collections::BTreeSet;
use specs::Component as SpecsComponent;
use specs::{Entities, Join, ReadStorage, System, VecStorage};
use crate::{
    prelude::{NodeVisitor, State},
    store::Store,
};

#[derive(Clone)]
pub enum NodeComponent {
    Input(fn() -> &'static str),
    Output(fn() -> &'static str),
}

impl SpecsComponent for NodeComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
pub struct EditorNode {
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

impl From<State> for EditorNode {
    fn from(state: State) -> Self {
        todo!()
    }
}

impl<'a> NodeVisitor<'a> for EditorNode {
    type Parameters = NodeComponent;

    fn dispatch(&self, params: Self::Parameters) -> Self {
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

impl<'a> System<'a> for EditorNode {
    type SystemData = (Entities<'a>, ReadStorage<'a, NodeComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let mut seen: BTreeSet<&'static str> = BTreeSet::new();
        let mut visited: BTreeSet<(Option<&'static str>, Option<&'static str>)> = BTreeSet::new();

        // loop through all components
        for (e, component) in (&data.0, &data.1).join() {
            let state: EditorNode = self.clone().dispatch(component.clone());
            // seen.clear();
            // visited.clear();

             state.config.walk_ordered("component", &mut seen, &mut visited);

            if !visited.insert((Some("component"), Some("input"))) {
                
            }

            if !visited.insert((Some("component"), Some("output"))) {
                
            }
        }
    }
}
