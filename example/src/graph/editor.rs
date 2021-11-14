use lib::*;

pub fn test<'a>() -> World {
    let mut w = World::new();

    let add_node = AddNode::<Editor, Data>::from(lib::Editor::default());

    let editor = lib::Editor::default();
    let mut dispatcher = DispatcherBuilder::new()
        .with(editor, "render", &[])
        .build(); 

    dispatcher.setup(&mut w);

    w.create_entity()
        .with(add_node)
        .build();

    dispatcher.dispatch(&w);
    w.maintain();

    return w;
}