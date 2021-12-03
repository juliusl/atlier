use crate::system::{Display, Attribute, NodeExterior, NodeResource, Reducer, Value};

pub struct ColorEditor;

impl NodeExterior for ColorEditor {
    fn title() -> &'static str {
        "Color Editor"
    }

    fn group_name() -> &'static str {
        "Graphics"
    }

    fn inputs() -> Option<Vec<NodeResource>> {
        Some(
            vec![
                RedChannel::parameter(),
                GreenChannel::parameter(),
                BlueChannel::parameter(),
                RedChannel::resource_input(),
                GreenChannel::resource_input(),
                BlueChannel::resource_input(),
            ]
        )
    }
}

impl Display for ColorEditor {
    fn display_name() -> &'static str {
        "preview"
    }
    
    fn display(
        name: String,
        width: f32,
        ui: &imgui::Ui,
        state: &crate::system::State,
    ) {
        if let (
            Some(Attribute::Literal(Value::Float(r))),
            Some(Attribute::Literal(Value::Float(g))),
            Some(Attribute::Literal(Value::Float(b))),
        ) = (state.get(RedChannel::param_name()), state.get(GreenChannel::param_name()), state.get(BlueChannel::param_name()))
        {
            let mut color = [r, g, b, 1.0];
            ui.set_next_item_width(width);
            imgui::ColorPicker::new(name, &mut color).display_rgb(true).build(ui); 
        }
    }
}

trait ChannelReducer : Reducer {
    fn reduce_rgba(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        if let Some(attr) = attribute {
            let float_value:f32 = attr.into();
            Some((float_value % 255.0).into())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct RedChannel;

impl NodeExterior for RedChannel {
    fn title() -> &'static str {
        "Red Channel"
    }

    fn group_name() -> &'static str {
        "Graphics"
    }
}

impl ChannelReducer for RedChannel {}
impl Reducer for RedChannel {
    fn param_name() -> &'static str {
        "red"
    }
    fn result_name() -> &'static str {
        "red_channel"
    }
    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
       Self::reduce_rgba(attribute)
    }
}

struct GreenChannel;
impl NodeExterior for GreenChannel {
    fn title() -> &'static str {
        "Green Channel"
    }

    fn group_name() -> &'static str {
        "Graphics"
    }
}
impl ChannelReducer for GreenChannel{}
impl Reducer for GreenChannel {
    fn param_name() -> &'static str {
        "green"
    }
    fn result_name() -> &'static str {
        "green_channel"
    }
    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        Self::reduce_rgba(attribute)
    }
}

struct BlueChannel;
impl NodeExterior for BlueChannel {
    fn title() -> &'static str {
        "Blue Channel"
    }

    fn group_name() -> &'static str {
        "Graphics"
    }
}

impl ChannelReducer for BlueChannel {}
impl Reducer for BlueChannel {
    fn param_name() -> &'static str {
        "blue"
    }
    fn result_name() -> &'static str {
        "blue_channel"
    }
    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        Self::reduce_rgba(attribute)
    }
}
