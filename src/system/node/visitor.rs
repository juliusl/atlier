use std::{collections::BTreeMap, hash::{Hash, Hasher}};

use imgui::TableColumnSetup;

use crate::system::Value;

use super::{AttributeValue, EditorResource};

// These are traits to help define different types of nodes that can be used from the editor 

pub trait NodeInterior<'a> {
    type Literal;
    type Visitor: NodeVisitor + From<Self::Literal>;

    // Accept a visitor to convert the current interior state
    fn accept(state: &'a BTreeMap<String, AttributeValue>) -> Self::Visitor;
}

pub trait NodeVisitor {
    // Evaluate a result from this visitor
    fn evaluate(&self) -> Option<AttributeValue>;
}

pub trait NodeExterior {
    // Return an editor resource to represent the exterior of the node
    fn resource(nodeid: Option<imnodes::NodeId>) -> EditorResource;
}

pub trait Reducer {
    // Implementation returns the parameter they expect
    fn param_name() -> &'static str;

    // Implementation reduces an attribute 
    fn reduce(attribute: Option<AttributeValue>) -> Option<AttributeValue>;

    // This will be called by the runtime in order to decide whether or not reduce should be called again
    fn map(state: &BTreeMap<String, AttributeValue>) -> (u64, Option<AttributeValue>) {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        if let Some(v) = state.get(Self::param_name()) {
            v.hash(&mut hasher);
            (hasher.finish(), Some(v.to_owned()))
        } else {
            (0, None)
        }
    }

    fn table_select(label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue) {
        if let AttributeValue::Map(map) = value {
            if let Some(table_token) = ui.begin_table_header_with_sizing(
                label,
                [
                    TableColumnSetup::new(Self::param_name()),
                    TableColumnSetup::new(""),
                ],
                imgui::TableFlags::RESIZABLE | imgui::TableFlags::SCROLL_Y, 
                [width, 300.0], 
                0.00
            ) {
                ui.spacing();
                for (key, value) in map {
                    ui.table_next_row();
                    ui.table_next_column();
                    if let AttributeValue::Literal(Value::Bool(selected)) = value {
                        if imgui::Selectable::new(key).span_all_columns(true).build_with_ref(ui, selected) {
                            ui.set_item_default_focus();
                        }
                    } else if let AttributeValue::Map(map) = value {
                        if let Some(AttributeValue::Literal(Value::Bool(selected))) = map.get_mut("selected") {
                            if imgui::Selectable::new(key).span_all_columns(true).build_with_ref(ui, selected) {
                                ui.set_item_default_focus();
                            }   
                        }
                    }
                }
                table_token.end();
            }
        }
    }
}