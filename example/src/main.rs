use std::panic;

use lib::*;

pub fn test<'a>() -> World {
    let mut w = World::new();

    let add_node = AddNode::<Editor>::from(lib::Editor::default());
    let data = Data::Add(lib::Add::default());

    let store = ContentStore::<Editor>::default();
    w.insert(store);

    let renderer = lib::AddEditorRenderer;
    let updater = lib::AddEditorUpdater;
    let mut dispatcher = DispatcherBuilder::new()
        .with(updater, "updater", &[])
        .with(renderer, "renderer", &["updater"])
        .build(); 

    dispatcher.setup(&mut w);

    let e = w.create_entity()
        .with(add_node)
        .with(data)
        .build();

    dispatcher.dispatch(&w);
    w.maintain();

    if let Err(e) = w.write_storage().insert(e, Data::Add(Add::new(10, 10))) {
      panic!("{}", e);
    }

    dispatcher.dispatch(&w);
    w.maintain();

    if let Err(e) = w.write_storage().insert(e, Data::Add(Add::new(10, 15))) {
      panic!("{}", e);
    }
    
    dispatcher.dispatch(&w);
    w.maintain();

    return w;
}

fn main() {
   test();
}