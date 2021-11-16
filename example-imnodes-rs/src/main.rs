
use atlier::prelude::new_gui_system;
use atlier::prelude::ControlState;
use atlier::prelude::GUIUpdate;
use lib::GraphApp;
use lib::SimpleNode;
use specs::prelude::*;
use winit::event_loop::ControlFlow;

fn main() {
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });

    let imnodes = imnodes::Context::new();

    let imnodes_editor = imnodes.create_editor();
    let idgen = imnodes_editor.new_identifier_generator();
   

    let other_editor = imnodes.create_editor();
    let other_idgen = other_editor.new_identifier_generator();

    let apps = vec![
        GraphApp{
            editor_context: imnodes_editor,
            id_gen: idgen,
            name: "Test1".to_string(),
            nodes: vec![
                SimpleNode::new("A".to_string(), 10.32),
                SimpleNode::new("B".to_string(), 12.43),
            ],
            links: vec![] 
        },

        GraphApp{
            editor_context: other_editor,
            id_gen: other_idgen,
            name: "Test2".to_string(),
            nodes: vec![
                SimpleNode::new("A".to_string(), 10.32),
                SimpleNode::new("B".to_string(), 12.43),
            ],
            links: vec![]
        },
    ];

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<GraphApp<SimpleNode>>("example-imnodes-specs", 1920.0, 1080.0, apps);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new()
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
