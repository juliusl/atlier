use std::collections::BTreeMap;

use crate::prelude::*;
use imgui::*;

pub struct ListDirectory;

// List Directory reduces a filepath to a list of files
// and outputs which files are selected in a table
impl Reducer for ListDirectory {
    fn param_name() -> &'static str {
        "filepath"
    }

    fn reduce(attribute: Option<AttributeValue>) -> Option<AttributeValue> {
        if let Some(AttributeValue::Literal(Value::TextBuffer(path))) = attribute {
            read_dir(&path)
        } else {
            None
        }
    }
}

// TODO: This trait could be derived
impl NodeExterior for ListDirectory {
    fn resource(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title("List Directory"),
                NodeResource::Attribute(
                    Self::param_name,
                    AttributeValue::input,
                    Some(AttributeValue::Literal(Value::TextBuffer("./".to_string()))),
                    None,
                ),
                NodeResource::Reducer(
                    || "selected files",
                    display_file_list, 
                    Self::map,
                    Self::reduce,
                    (0, None),
                    None,
                    None,
                )
            ],
            id: nodeid
        }
    }
}

fn read_dir(path: &str) -> Option<AttributeValue> {
    if let Ok(paths) = std::fs::read_dir(path) {
            let mut map = BTreeMap::<String, AttributeValue>::new();
            for path in paths {
                if let Ok(dir_entry) = path {
                    if let (Some(path), Ok(..)) = (
                        dir_entry.file_name().to_str(),
                        dir_entry.metadata(),
                    ) {
                        map.insert(
                            path.to_string(),
                            AttributeValue::Literal(Value::Bool(false))
                        );
                    }
                }
            }
            Some(AttributeValue::Map(map))
        } else {
        None
    }
}

fn display_file_list(label: String, width: f32, ui: &imgui::Ui, value: &mut AttributeValue) {
    if let AttributeValue::Map(map) = value {
        if let Some(table_token) = ui.begin_table_header_with_sizing(
            label,
            [
                TableColumnSetup::new("filename"),
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
                }
            }
            table_token.end();
        }
    }
}