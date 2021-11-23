use crate::system::{AttributeValue, EditorResource, NodeExterior, NodeResource, Reducer, Value};

pub struct ColorEditor{}

impl NodeExterior for ColorEditor {
    fn resource(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource {
        EditorResource::Node {
            id: nodeid,
            resources: vec![
                NodeResource::Title("RGBA"),
                NodeResource::Input(||"red", None),
                NodeResource::Input(||"green", None),
                NodeResource::Input(||"blue", None),
                NodeResource::Reducer(||"r", |_,_,_,_|{}, RedChannel::map, RedChannel::reduce, (0, None), None, None),
                NodeResource::Reducer(||"g", |_,_,_,_|{}, BlueChannel::map, BlueChannel::reduce, (0, None), None, None),
                NodeResource::Reducer(||"b", |_,_,_,_|{}, GreenChannel::map, GreenChannel::reduce, (0, None), None, None),
                NodeResource::Action(||"display", 
                |name, width, ui, map|{
                    if let (
                        Some(AttributeValue::Literal(Value::Float(r))), 
                        Some(AttributeValue::Literal(Value::Float(g))), 
                        Some(AttributeValue::Literal(Value::Float(b)))) = (map.get("r"), map.get("g"), map.get("b")) {
                            let mut color = [*r, *g, *b, 1.0];
                            ui.set_next_item_width(width);
                            imgui::ColorPicker::new(name, &mut color).build(ui);
                    }

                    Some(AttributeValue::Map(map.to_owned()))
                },
                None, 
                None)
            ],
        }
    }
}


struct RedChannel;
impl Reducer for RedChannel {
    fn param_name() -> &'static str {
        "red"
    }

    fn reduce(attribute: Option<crate::system::AttributeValue>) -> Option<crate::system::AttributeValue> {
        if let Some(AttributeValue::Literal(value)) = attribute {
            Some(Value::Float(rgba_color_value(value)).into())
        } else {
            None
        }
    }
}

struct GreenChannel;
impl Reducer for GreenChannel {
    fn param_name() -> &'static str {
        "green"
    }

    fn reduce(attribute: Option<crate::system::AttributeValue>) -> Option<crate::system::AttributeValue> {
        if let Some(AttributeValue::Literal(value)) = attribute {
            Some(Value::Float(rgba_color_value(value)).into())
        } else {
            None
        }
    }
}

struct BlueChannel;
impl Reducer for BlueChannel {
    fn param_name() -> &'static str {
        "blue"
    }

    fn reduce(attribute: Option<crate::system::AttributeValue>) -> Option<crate::system::AttributeValue> {
        if let Some(AttributeValue::Literal(value)) = attribute {
            Some(Value::Float(rgba_color_value(value)).into())
        } else {
            None
        }
    }
}

fn rgba_color_value(v: Value) -> f32 {
    match v {
        crate::system::Value::Float(f) => f % 255.0,
        crate::system::Value::Int(i) => (i as f32) % 255.0,
        crate::system::Value::FloatRange(f, _, _) => f % 255.0,
        crate::system::Value::IntRange(i, _, _) => (i as f32) % 255.0,
        _ => 0.00
    }
}