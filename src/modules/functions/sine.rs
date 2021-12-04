use crate::system::{NodeExterior, Reducer};

pub struct Sine;

impl NodeExterior for Sine {
    fn title() -> &'static str {
        "Sine"
    }

    fn group_name() -> &'static str {
        "Functions"
    }
}

impl Reducer for Sine {
    fn param_name() -> &'static str {
        "input"
    }

    fn result_name() -> &'static str {
        "sin(input)"
    }

    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        if let Some(attr) = attribute {
            let float_value: f32 = attr.into();
            Some(float_value.sin().into())
        } else {
            None
        }
    }

    fn display(label: String, width: f32, ui: &imgui::Ui, value: &mut crate::system::Attribute) {
        Self::noop_display(label, width, ui, value)
    }
}
