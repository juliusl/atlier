use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, World, WorldExt, EntityBuilder, Entities};
use winit::event_loop::{ControlFlow, EventLoop};

use super::{new_gui_system, user_app::{Plugin, PluginState}, ControlState, GUIUpdate, UserApp};

pub struct Runtime<'a> {
    dispatcher: Dispatcher<'a, 'static>,
    event_loop: EventLoop<()>,
    gui_entity: Entity,
    world: World,
    user_app: &'static UserApp,
    plugins: Vec<Plugin>,
}

impl Runtime<'static> {
    pub fn new(title: &'static str, width: f64, height: f64, user_app: &'static UserApp) -> Self {
        let mut world = World::new();
        world.insert(ControlState { control_flow: None });

        // Create the new gui_system,
        // after this point no changes can be made to gui or event_loop
        // This application either starts up, or panics here
        let (event_loop, gui) = new_gui_system(title, width, height);

        // Create the specs dispatcher
        let mut dispatcher = DispatcherBuilder::new().with_thread_local(gui).build();

        dispatcher.setup(&mut world);

        // Create a gui entity that we can use to communicate with the window
        let gui_entity = world
            .create_entity()
            .maybe_with(Some(GUIUpdate {
                user_app: UserApp {},
                event: winit::event::Event::Suspended,
            }))
            .build();

        Runtime {
            gui_entity,
            dispatcher,
            event_loop,
            world,
            user_app,
            plugins: vec![]
        }
    }

    pub fn add_plugin(mut self, plugin: Plugin) -> Self {
        self.plugins.push(plugin);

        self
    }

    pub fn start(self)
    {
        let mut dispatcher = self.dispatcher;
        let gui_entity = self.gui_entity;
        let mut w = self.world;
        // Starts the event loop
        self.event_loop.run(move |event, _, control_flow| {
            for plugin in &self.plugins {
                plugin.update(&mut w, PluginState{});
            }

            dispatcher.dispatch(&w);
            w.maintain();

            // THREAD LOCAL
            // Dispatch the next event to the gui_entity that is rendering windows
            if let Some(event) = event.to_static() {
                if let Err(err) = w.write_component().insert(
                    gui_entity,
                    GUIUpdate {
                        event,
                        user_app: self.user_app.apply(&w),
                    },
                ) {
                    println!("Error: {}", err)
                } else {
                    dispatcher.dispatch_thread_local(&w);
                }
            }

            w.maintain();

            // The gui_system can dispatch back some control state, which we can read here
            let control_state = w.read_resource::<ControlState>();
            if let Some(c) = control_state.control_flow {
                *control_flow = c;
            } else {
                *control_flow = ControlFlow::Poll;
            }
        });
    }
}
