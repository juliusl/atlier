use std::collections::BTreeMap;

use crate::prelude::*;
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
                    Self::table_select, 
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
                        let mut filedata = BTreeMap::<String, AttributeValue>::new();

                        filedata.insert("selected".to_string(), 
                        AttributeValue::Literal(Value::Bool(false)));

                        filedata.insert("filepath".to_string(),
                        AttributeValue::Literal(Value::TextBuffer(dir_entry.path().to_string_lossy().to_string())));

                        map.insert(
                           path.to_string(),
                            AttributeValue::Map(filedata),
                        );
                    }
                }
            }
            Some(AttributeValue::Map(map))
        } else {
        None
    }
}
