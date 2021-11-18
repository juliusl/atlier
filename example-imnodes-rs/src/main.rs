
use atlier::prelude::new_gui_system;
use atlier::prelude::ControlState;
use atlier::prelude::GUIUpdate;
use atlier::system::AttributeValue;
use atlier::system::EditorResource;
use atlier::system::NodeApp;
use atlier::system::Sum;
use atlier::prelude::NodeResource;
use specs::prelude::*;
use winit::event_loop::ControlFlow;

fn main() {
    let attr = NodeResource::Attribute(
        || "test", 
        AttributeValue::slider,
        Some(AttributeValue::Float(10.0, 0.0, 100.0)),
        None);

    let mut w = World::new();
    w.insert(ControlState { control_flow: None });

    let app = NodeApp::new("node-app".to_string())
        .with(vec![
            Sum::default().into(), 
            Sum::default().into(), 
            Sum::default().into(), 
            Sum::default().into(),
            EditorResource::Node {
                resources: vec![
                    NodeResource::Title("hello"),
                    attr,
                    NodeResource::Output(||"output", None),
                ],
                id: None,
            }
        ]);

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<NodeApp>("example-imnodes-specs", 1920.0, 1080.0, vec![
            app,
        ]);

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
