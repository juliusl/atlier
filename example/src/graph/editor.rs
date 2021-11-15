use std::any::Any;

use lib::*;

pub fn test<'a>(typeid: std::any::TypeId) -> World {
    let mut w = World::new();

    let add_node = AddNode::<Editor>::from(lib::Editor::default());
    let data = Data::Add(lib::Add::default());

    let lhs_content_id = add_node.get_attributes().elems[0].content_id.clone();

    let mut store = ContentStore::<Editor>::default();
    store.set(lhs_content_id, &EditorData::Integer(10));
    w.insert(store);

    let editor = lib::Editor::default();
    let mut dispatcher = DispatcherBuilder::new()
        .with(editor, "render", &[])
        .build(); 

    dispatcher.setup(&mut w);

    let e = w.create_entity()
        .with(add_node)
        .with(data)
        .build();
    dispatcher.dispatch(&w);
    w.maintain();

    return w;
}