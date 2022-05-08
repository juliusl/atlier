use imgui::{ ChildWindow, InputText, Window};

use super::App;

#[derive(Clone)]
pub struct Test {
    test_data: String
}

impl Default for Test {
    fn default() -> Self {
        Self { test_data: Default::default() }
    }
}

impl Test {
    pub fn name() -> String {
        "test-window".to_string()
    }
}

impl App for Test {
    fn show(&mut self, ui: &imgui::Ui) {
        ChildWindow::new("test").size([-1.0, 0.00]).build(ui, || {

            InputText::new(ui, "test-text", &mut self.test_data).build();

        });
    }
}