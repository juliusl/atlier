use atlier::prelude::*;

#[derive(Clone)]
struct WorldData { 
    str_a: String
}

impl Default for WorldData {
    fn default() -> Self {
        Self { str_a: Default::default() }
    }
}

fn main() {
    start_editor::<WorldData>(|ui, state|{
        use imgui::ChildWindow;
        use imgui::InputText;

        let mut next_str_a = state.str_a.clone();
        let next_str_a = &mut next_str_a;
        ChildWindow::new("test").size([-1.0, 0.00]).build(ui, || {
            InputText::new(ui, "test", next_str_a).build();
        });

        let mut next = state.clone();
        next.str_a = next_str_a.clone();
        next
    });
}
