use atlier::prelude::{NodeEditor, new_gui_system, GUIUpdate, ControlState};
use specs::prelude::*;
use winit::event_loop::ControlFlow;

fn main() {
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<NodeEditor>("example-imnodes-specs", 1920.0, 1080.0, vec![]);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(atlier::prelude::EditorNode::default(), "editor_node", &[])
        .with_thread_local(gui)
        .build();
    dispatcher.setup(&mut w);

    // Create a gui entity that we can use to communicate with the window
    let gui_entity = w
        .create_entity()
        .maybe_with(Some(GUIUpdate {
            event: winit::event::Event::Suspended,
        }))
        .build();

    w.create_entity().with(atlier::modules::NodeComponent::Input(|| "test")).build();
    w.create_entity().with(atlier::modules::NodeComponent::Output(|| "test")).build();

    // Starts the event loop
    event_loop.run(move |event, _, control_flow| {
        // LOGIC

        dispatcher.dispatch_seq(&w);
        w.maintain();

        // THREAD LOCAL
        // Dispatch the next event to the gui_entity that is rendering windows
        if let Some(event) = event.to_static() {
            if let Err(err) = w
                .write_component()
                .insert(gui_entity, GUIUpdate { event: event })
            {
                println!("Error: {}", err)
            }
            dispatcher.dispatch_thread_local(&w);
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
