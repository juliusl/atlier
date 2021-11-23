use atlier::prelude::*;
use specs::prelude::*;
use std::{collections::BTreeMap};
use winit::event_loop::ControlFlow;

fn main() {
    let attr = NodeResource::Attribute(
        || "test",
        AttributeValue::input,
        Some(Value::FloatRange(10.0, 0.0, 100.0).into()),
        None,
    );

    let attrstr = NodeResource::Attribute(
        || "test-str",
        AttributeValue::input,
        Some(Value::TextBuffer(String::new()).into()),
        None,
    );

    let attrfloat = NodeResource::Attribute(
        || "test-float",
        AttributeValue::input,
        Some(Value::Float(0.0).into()),
        None,
    );

    let mut w = World::new();
    w.insert(ControlState { control_flow: None });

    let app = NodeEditor::new("node-editor".to_string()).module(
        vec![
            FloatExpression::<Add>::resource(None),
            FloatExpression::<Subtract>::resource(None),
            FloatExpression::<Divide>::resource(None),
            FloatExpression::<Multiply>::resource(None),
            ListDirectory::resource(None),
            EditorResource::merge(&FloatExpression::<Multiply>::resource(None), ListDirectory::resource(None)),
            EditorResource::Node {
                resources: vec![
                    NodeResource::Title("hello"),
                    attr,
                    attrstr,
                    attrfloat,
                    NodeResource::Output(
                        || "output",
                        |_| Some(Value::Float(5.0).into()),
                        None,
                        None,
                    ),
                ],
                id: None,
            },
            EditorResource::Node {
                resources: Test {
                    lhs: Value::FloatRange(10.0, 0.0, 100.0).into(),
                    rhs: Value::FloatRange(10.0, 0.0, 100.0).into(),
                }
                .node(),
                id: None,
             },
        ],
        true,
    );

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<NodeEditor>("example-imnodes-specs", 1920.0, 1080.0, vec![app]);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new().with_thread_local(gui).build();
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

struct Test {
    lhs: AttributeValue,
    rhs: AttributeValue,
}

impl Test {
    fn node(&mut self) -> Vec<NodeResource> {
        let mut map: BTreeMap<String, AttributeValue> = BTreeMap::new();

        map.insert("string".to_string(), Value::Bool(false).into());
        map.insert("int32".to_string(), Value::Bool(false).into());
        map.insert("float32".to_string(), Value::Bool(false).into());

        let mut settings: BTreeMap<String, AttributeValue> = BTreeMap::new();

        settings.insert(
            "name".to_string(),
            Value::TextBuffer(String::default()).into(),
        );
        settings.insert("fields".to_string(), Value::Int(0).into());
        settings.insert("render".to_string(), Value::Bool(false).into());

        let mut settings2: BTreeMap<String, AttributeValue> = BTreeMap::new();

        settings2.insert(
            "nested_name".to_string(),
            Value::TextBuffer(String::default()).into(),
        );
        settings2.insert("nested_fields".to_string(), Value::Int(0).into());
        settings2.insert("nested_render".to_string(), Value::Bool(false).into());

        settings.insert("nested".to_string(), AttributeValue::Map(settings2));

        vec![
            NodeResource::Title("Read Directory"),
            NodeResource::Attribute(
                || "lhs",
                AttributeValue::input,
                Some(self.lhs.to_owned()),
                None,
            ),
            NodeResource::Attribute(
                || "rhs",
                AttributeValue::input,
                Some(self.rhs.to_owned()),
                None,
            ),
            NodeResource::Attribute(
                || "types",
                AttributeValue::select,
                Some(AttributeValue::Map(map)),
                None,
            ),
            NodeResource::Attribute(
                || "settings",
                AttributeValue::input,
                Some(AttributeValue::Map(settings)),
                None,
            ),
            NodeResource::Attribute(
                || "test_check_box",
                AttributeValue::input,
                Some(AttributeValue::Literal(Value::Bool(false))),
                None,
            ),
            NodeResource::Output(
                || "output",
                |state| {
                    if let (
                        Some(AttributeValue::Literal(Value::FloatRange(lhs, ..))),
                        Some(AttributeValue::Literal(Value::FloatRange(rhs, ..))),
                    ) = (state.get("lhs"), state.get("rhs"))
                    {
                        return Some(Value::Float(lhs + rhs).into());
                    }
                    None
                },
                None,
                None,
            ),
            // NodeResource::OutputWithAttribute(
            //     || "output_with_attr",
            //     |label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue| {
            //         if let AttributeValue::Literal(Value::Float(f)) = value {
            //             ui.set_next_item_width(width);
            //             imgui::InputFloat::new(ui, label, f).read_only(true).build();
            //         }
            //     },
            //     |state| {
            //         if let Some(v) = state.get("output") {
            //             return Some(v.clone());
            //         }
            //         None
            //     },
            //     None,
            //     None,
            //     None,
            // ),
            NodeResource::Attribute(
                || "filepath",
                AttributeValue::input,
                Some(AttributeValue::Literal(Value::TextBuffer("./".to_string()))),
                None,
            ),
            // NodeResource::Action(
            //     || "internals",
            //     display_internals,
            //     None,
            //     None,
            // ), 
        ]
    }
}
