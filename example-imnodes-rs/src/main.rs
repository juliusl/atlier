use atlier::prelude::*;
use std::collections::BTreeMap;

fn main() {
    let runtime = Runtime::new(
        "example-imnodes-rs", 
        1920.0, 
        1080.0, 
        &UserApp{});

    runtime.start()
}

struct Test {
    lhs: AttributeValue,
    rhs: AttributeValue,
}

impl Test {
    fn node(&mut self) -> Vec<NodeResource> {
        let mut map: BTreeMap<String, AttributeValue> = BTreeMap::new();

        map.insert("string".to_string(), Value::Bool(false).into());
        map.insert("int32".to_string(), Value::Bool(false).into());
        map.insert("float32".to_string(), Value::Bool(false).into());

        let mut settings: BTreeMap<String, AttributeValue> = BTreeMap::new();

        settings.insert(
            "name".to_string(),
            Value::TextBuffer(String::default()).into(),
        );
        settings.insert("fields".to_string(), Value::Int(0).into());
        settings.insert("render".to_string(), Value::Bool(false).into());

        let mut settings2: BTreeMap<String, AttributeValue> = BTreeMap::new();

        settings2.insert(
            "nested_name".to_string(),
            Value::TextBuffer(String::default()).into(),
        );
        settings2.insert("nested_fields".to_string(), Value::Int(0).into());
        settings2.insert("nested_render".to_string(), Value::Bool(false).into());

        settings.insert("nested".to_string(), AttributeValue::Map(settings2));

        vec![
            NodeResource::Title("Read Directory"),
            NodeResource::Attribute(
                || "lhs",
                AttributeValue::input,
                Some(self.lhs.to_owned()),
                None,
            ),
            NodeResource::Attribute(
                || "rhs",
                AttributeValue::input,
                Some(self.rhs.to_owned()),
                None,
            ),
            NodeResource::Attribute(
                || "types",
                AttributeValue::select,
                Some(AttributeValue::Map(map)),
                None,
            ),
            NodeResource::Attribute(
                || "settings",
                AttributeValue::input,
                Some(AttributeValue::Map(settings)),
                None,
            ),
            NodeResource::Attribute(
                || "test_check_box",
                AttributeValue::input,
                Some(AttributeValue::Literal(Value::Bool(false))),
                None,
            ),
            NodeResource::Output(
                || "output",
                |state| {
                    if let (
                        Some(AttributeValue::Literal(Value::FloatRange(lhs, ..))),
                        Some(AttributeValue::Literal(Value::FloatRange(rhs, ..))),
                    ) = (state.get("lhs"), state.get("rhs"))
                    {
                        return Some(Value::Float(lhs + rhs).into());
                    }
                    None
                },
                None,
                None,
            ),
            // NodeResource::OutputWithAttribute(
            //     || "output_with_attr",
            //     |label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue| {
            //         if let AttributeValue::Literal(Value::Float(f)) = value {
            //             ui.set_next_item_width(width);
            //             imgui::InputFloat::new(ui, label, f).read_only(true).build();
            //         }
            //     },
            //     |state| {
            //         if let Some(v) = state.get("output") {
            //             return Some(v.clone());
            //         }
            //         None
            //     },
            //     None,
            //     None,
            //     None,
            // ),
            NodeResource::Attribute(
                || "filepath",
                AttributeValue::input,
                Some(AttributeValue::Literal(Value::TextBuffer("./".to_string()))),
                None,
            ),
            // NodeResource::Action(
            //     || "internals",
            //     display_internals,
            //     None,
            //     None,
            // ),
        ]
    }
}
