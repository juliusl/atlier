use atlier::prelude::*;
use imgui::{
    TableColumnSetup,
};
use specs::prelude::*;
use std::{collections::BTreeMap, hash::Hasher};
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

    let app = NodeApp::new("node-app".to_string()).module(
        vec![
            FloatExpression::<Add>::resource(None),
            FloatExpression::<Subtract>::resource(None),
            FloatExpression::<Divide>::resource(None),
            FloatExpression::<Multiply>::resource(None),
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
            // EditorResource::Node {
            //     resources: Test {
            //         lhs: Value::FloatRange(10.0, 0.0, 100.0).into(),
            //         rhs: Value::FloatRange(10.0, 0.0, 100.0).into(),
            //     }
            //     .node(),
            //     id: None,
            // },
            // EditorResource::Node {
            //     resources: vec![
            //         NodeResource::Title("select"),
            //         NodeResource::Input(||"items", None),
            //         NodeResource::Attribute(||"filter", AttributeValue::input, Some(Value::TextBuffer(String::new()).into()), None),
            //         NodeResource::Output(
            //             || "selected",
            //             |state| {
            //                 if let (
            //                     Some(AttributeValue::Literal(Value::TextBuffer(filter))), 
            //                     Some(AttributeValue::Map(m))) = (state.get("filter"), state.get("items")) {
                                
            //                     if let Some(found) = m.iter().find(|f| f.0 == filter) {
            //                         Some(found.1.clone())
            //                     } else {
            //                         None
            //                     }
            //                 } else {
            //                     None
            //                 }
            //             },
            //             None,
            //             None
            //         ),
            //         NodeResource::Action(
            //             || "internals",
            //             display_internals,
            //             None,
            //             None,
            //         ),
            //     ],
            //     id: None,
            // },
            // EditorResource::Node {
            //     resources: vec![
            //         NodeResource::Title("view file"),
            //         NodeResource::Input(||"filepath", None),
            //         NodeResource::Action(||"contents", 
            //         |label, width, ui, state| {
            //             if let Some(path) = state.get("filepath") {
            //                 display_file(label, width, ui, &mut path.to_owned());
            //             }

            //             Some(AttributeValue::Map(state.to_owned()))
            //         }, None, None),
            //         NodeResource::Action(
            //             || "internals",
            //             display_internals,
            //             None,
            //             None,
            //         ),
            //     ],
            //     id: None,
            // }
        ],
        false,
    );

    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    let (event_loop, gui) =
        new_gui_system::<NodeApp>("example-imnodes-specs", 1920.0, 1080.0, vec![app]);

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
            NodeResource::OutputWithAttribute(
                || "files",
                display_file_list,
                |state| {
                    if let Some(AttributeValue::Literal(Value::TextBuffer(path))) = state.get("filepath")
                    {
                        let mut outside_hasher = std::collections::hash_map::DefaultHasher::default();
                        let next = read_dir(path);
                        next.hash(&mut outside_hasher);
                        if let Some(v) = state.get("files") {
                            let mut inside_hasher = std::collections::hash_map::DefaultHasher::default();
                            v.hash(&mut inside_hasher);

                            if outside_hasher.finish() == inside_hasher.finish() {
                                return Some(v.to_owned())
                            } else {
                                return next;
                            }
                        } else {
                            return next;
                        }
                    } else {
                        Some(AttributeValue::Map(BTreeMap::new()))
                    }
                },
                None,
                None,
                None,
            ),
            NodeResource::Action(
                || "internals",
                display_internals,
                None,
                None,
            ), 
        ]
    }
}

fn display_file_list(label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue) {
    if let AttributeValue::Map(map) = value {
        if let Some(table_token) = ui.begin_table_header_with_sizing(
            label,
            [
                TableColumnSetup::new("filename"),
                TableColumnSetup::new(""),
            ],
            imgui::TableFlags::RESIZABLE | imgui::TableFlags::SCROLL_Y, 
            [width*2.0, 300.0], 
            0.00
        ) {
            ui.spacing();


            for (key, _) in map {
                ui.table_next_row();
                ui.table_next_column();
                ui.text(key);

                // if let AttributeValue::Literal(Value::Bool(selected)) = value {
                //     if imgui::Selectable::new(key).span_all_columns(true).build_with_ref(ui, selected) {
                //         ui.set_item_default_focus();
                //     }
                //     // ui.table_next_column();
                //     // if let Some(AttributeValue::Literal(Value::TextBuffer(created))) = m.get("created") {
                //     //     ui.text(format!("{:?}", created))
                //     // }
                // }
            }
            table_token.end();
        }
    }
}
use time::OffsetDateTime;

fn read_dir(path: &str) -> Option<AttributeValue> {
    if let Ok(paths) = std::fs::read_dir(path) {
            let mut map = BTreeMap::<String, AttributeValue>::new();
            for path in paths {
                if let Ok(dir_entry) = path {
                    if let (Some(path), Ok(metadata)) = (
                        dir_entry.file_name().to_str(),
                        dir_entry.metadata(),
                    ) {
                        map.insert(
                            path.to_string(),
                            AttributeValue::Literal(Value::TextBuffer(dir_entry.path().display().to_string()))
                        );

                        // if let Ok(created) = metadata.created() {
                            // let mut file_info =  BTreeMap::new();
                            // let created = OffsetDateTime::from(created);
                            // let created = Value::TextBuffer(
                            //     format!("{}-{}-{} {}:{}", created.year(), created.month(), created.day(), created.hour(), created.minute())
                            // );
                            // file_info.insert("created".to_string(), AttributeValue::Literal(created));
    
                        // }
                    }
                }
            }

            Some(AttributeValue::Map(map))
        } else {
        None
    }
}

fn display_internals(name: String, width:f32, ui: &imgui::Ui, state: &BTreeMap<String, AttributeValue>) -> Option<AttributeValue> {
    ui.spacing();

    if let Some(table_token) = ui.begin_table_header_with_sizing(name, 
        [imgui::TableColumnSetup::new("property"), imgui::TableColumnSetup::new("value")], 
        imgui::TableFlags::RESIZABLE | imgui::TableFlags::SCROLL_Y, 
        [width*2.0, 300.0], 
        0.00) {
            for (l, v) in state {
                ui.table_next_row();
                ui.table_next_column();
                imgui::TreeNode::new(l).build(ui, || {
                    ui.table_next_column();
                    ui.text(format!("{:#?}", v));
                });
            }

            table_token.end();
        }

    Some(AttributeValue::Map(state.to_owned()))
}

fn display_file(label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue) {
    if let AttributeValue::Literal(Value::TextBuffer(path)) = value {
        if std::path::Path::new(path).exists() {
            if let Ok(mut contents) = std::fs::read_to_string(path){
                ui.input_text_multiline(label, &mut contents, [width, 0.00]).build();
            }
        }
    }
}