use atlier::system::{App, Extension};
use specs::System;

fn main() {
    atlier::prelude::open_window(
        "",
        Demo::default(),
        DemoExtension::default(),
        None,
        None,
    )
}

#[derive(Default)]
struct Demo;

impl<'a> System<'a> for Demo {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        
    }
}

impl App for Demo {
    fn name() -> &'static str {
        "demo"
    }

    fn edit_ui(&mut self, _ui: &imgui::Ui) {}

    fn display_ui(&self, ui: &imgui::Ui) {
        ui.main_menu_bar(|| {
            ui.menu("File", || {
                ui.menu("Test", || {

                })
            })
        });

        ui.window("Test").build(|| {
            ui.text("Hello World");
        });
    }
}

#[derive(Default)]
struct DemoExtension;

impl Extension for DemoExtension {}
