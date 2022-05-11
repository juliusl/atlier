use atlier::prelude::*;

#[derive(Clone)]
struct WorldData {
    str_a: String,
}

impl Default for WorldData {
    fn default() -> Self {
        Self {
            str_a: Default::default(),
        }
    }
}

fn main() {
    start_editor::<WorldData>(
        "example",
        1920.0,
        1080.0,
        WorldData {
            str_a: "test".to_string(),
        },
        |ui, state, _| {
            use imgui::Window;
            use imgui::InputText;

            ui.show_demo_window(&mut true);

            let mut next_str_a = state.str_a.clone();
            let next_str_a = &mut next_str_a;
            Window::new("test").size([1920.0, 1080.0], imgui::Condition::FirstUseEver).build(ui, || {
                InputText::new(ui, "test", next_str_a).build();
            });

            let mut next = state.clone();
            next.str_a = next_str_a.clone();
            Some(next)
        },
        false
    );
}
