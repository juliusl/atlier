use crate::system::{Attribute, EditorResource, NodeExterior, NodeResource, Reducer, Value};

pub struct ColorEditor {}

impl NodeExterior for ColorEditor {
    fn resource(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource {
        EditorResource::Node {
            id: nodeid,
            resources: vec![
                NodeResource::Title(ColorEditor::title()),
                NodeResource::Input(|| "red", None),
                NodeResource::Input(|| "green", None),
                NodeResource::Input(|| "blue", None),
                NodeResource::Reducer(
                    RedChannel::result_name,
                    Self::input,
                    RedChannel::map,
                    RedChannel::reduce,
                    (0, None),
                    None,
                    None,
                ),
                NodeResource::Reducer(
                    BlueChannel::result_name,
                    Self::input,
                    BlueChannel::map,
                    BlueChannel::reduce,
                    (0, None),
                    None,
                    None,
                ),
                NodeResource::Reducer(
                    GreenChannel::result_name,
                    Self::input,
                    GreenChannel::map,
                    GreenChannel::reduce,
                    (0, None),
                    None,
                    None,
                ),
                NodeResource::Action(|| "display", Self::action, None, None),
            ],
        }
    }

    fn action(
        name: String,
        width: f32,
        ui: &imgui::Ui,
        state: crate::system::State,
    ) -> Option<Attribute> {
        if let (
            Some(Attribute::Literal(Value::Float(r))),
            Some(Attribute::Literal(Value::Float(g))),
            Some(Attribute::Literal(Value::Float(b))),
        ) = (state.get("r"), state.get("g"), state.get("b"))
        {
            let mut color = [*r, *g, *b, 1.0];
            ui.set_next_item_width(width);
            imgui::ColorPicker::new(name, &mut color).build(ui);
        }

        Some(state.into())
    }

    fn title() -> &'static str {
        "Color Editor"
    }
}

trait ChannelReducer : Reducer {
    fn rgba_color_value(v: Value) -> f32 {
        match v {
            crate::system::Value::Float(f) => f % 255.0,
            crate::system::Value::Int(i) => (i as f32) % 255.0,
            crate::system::Value::FloatRange(f, _, _) => f % 255.0,
            crate::system::Value::IntRange(i, _, _) => (i as f32) % 255.0,
            _ => 0.00
        }
    }

    fn reduce_rgba(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        if let Some(Attribute::Literal(value)) = attribute {
            Some(Value::Float(Self::rgba_color_value(value)).into())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct RedChannel;

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
impl ChannelReducer for BlueChannel {}
impl Reducer for BlueChannel {
    fn param_name() -> &'static str {
        "blue"
    }
    fn result_name() -> &'static str {
        "blue_channe;"
    }
    fn reduce(attribute: Option<crate::system::Attribute>) -> Option<crate::system::Attribute> {
        Self::reduce_rgba(attribute)
    }
}
