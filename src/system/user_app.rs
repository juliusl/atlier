use specs::{World, Component, Entity, HashMapStorage, WorldExt};


pub struct PluginState {}

impl Component for PluginState {
    type Storage = HashMapStorage<Self>;
}

pub struct Plugin {
    entity: Entity
}

impl Plugin {
    pub fn update<T>(&self, world: &mut World, component: T)
    where
        T: Component
    {
        if let Err(err) = world.write_component().insert(self.entity, component) {
            eprintln!("err: Could not update entity, {}", err)
        }
    }
}

#[derive(Clone, Copy)]
pub struct UserApp;

impl UserApp {
    pub fn apply(&self, world: &World) -> Self
    {
        // TODO: 
        // Apply current state of the world to user's app
        // Return an instance of result
        // Show will render appropriately
        UserApp{}
    }

    pub fn show(&self, ui: &imgui::Ui) {

        // TODO:
        // Render the result from apply
        // 
        todo!()
    }
}

