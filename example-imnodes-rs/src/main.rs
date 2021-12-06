use atlier::prelude::*;
use specs::prelude::*;
use winit::event_loop::ControlFlow;

struct Color {
    r: f32,
    g: f32,
    b: f32, 
}

fn main() {
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });

    let state = State::default()
        .insert("lhs", 0.00)
        .insert("rhs", 0.00)
        .select("lhs", |state| 
            state.visit::<Initializer>().returns("lhs"))
        .select("rhs", |state| 
            state
            .visit::<Initializer>()
            .returns("rhs"));

    let state =   State::default()
        .insert("blue", [0.00, 0.00, 255.00])
        .insert("green", [0.00, 0.00, 255.00])
        .insert("red", [0.00, 0.00, 255.00]);

    let color_editor = state
            .map(&["red", "green", "blue"])
            .visit::<ColorEditor>()
            .call("display");

    let app = NodeEditor::new("node-editor").module(
        vec![
            FloatExpression::<Add>::output_node(None),
            //ListDirectory::reducer_node(None, true),
            //color_editor
            //ColorEditor::editor_resource(None),
            //ColorEditor::display_node(None),
        ],
        true,
    );

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<NodeEditor>("example-imnodes-specs", 1920.0, 1080.0, vec![app]);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(color_editor, "color_editor", &[])
        .with_thread_local(gui).build();
    dispatcher.setup(&mut w);

    // Create a gui entity that we can use to communicate with the window
    let gui_entity = w
        .create_entity()
        .maybe_with(Some(GUIUpdate {
            event: winit::event::Event::Suspended,
        }))
        .build();

    // let expr_entity = w
    //     .create_entity()
    //     .with(atlier::prelude::Resource::new(FloatExpression::<Add>::new()))
    //     .build();

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
