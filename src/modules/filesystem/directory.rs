use std::collections::BTreeMap;
use specs::{Component, DenseVecStorage, Entities, Join, ReadStorage, System, WriteStorage};

use crate::prelude::*;

pub struct ListDirectory;

impl Component for ListDirectory {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> System<'a> for ListDirectory {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Resource<ListDirectory>>, 
        WriteStorage<'a, Resource<ListDirectory>>);

    fn run(&mut self, (e, existing, mut resources): Self::SystemData) {
        (&e, existing.maybe()).join().for_each(|(e, r)| {
            if let None = r {
                if let Err(e) = resources.insert(e, Resource::new(ListDirectory{})) {
                    println!("Error: {}", e);
                }
            }
        }); 
    }
}

// List Directory reduces a filepath to a list of files
// and outputs which files are selected in a table
impl Reducer for ListDirectory {
    fn param_name() -> &'static str {
        "filepath"
    }

    fn result_name() -> &'static str {
        "selected files"
    }

    fn reduce(attribute: Option<Attribute>) -> Option<Attribute> {
        if let Some(Attribute::Literal(Value::TextBuffer(path))) = attribute {
            read_dir(&path)
        } else {
            None
        }
    }
}

// TODO: This trait could be derived
impl NodeExterior for ListDirectory {
    fn title() -> &'static str {
        "List Directory"
    }

    fn resource(nodeid: Option<imnodes::NodeId>) -> EditorResource {
        EditorResource::Node {
            resources: vec![
                NodeResource::Title(Self::title()),
                NodeResource::Attribute(
                    Self::param_name,
                    Self::input,
                    Some(Attribute::Literal(Value::TextBuffer("./".to_string()))),
                    None,
                ),
                NodeResource::Reducer(
                    Self::result_name,
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

fn read_dir(path: &str) -> Option<Attribute> {
    if let Ok(paths) = std::fs::read_dir(path) {
            let mut map = BTreeMap::<String, Attribute>::new();
            for path in paths {
                if let Ok(dir_entry) = path {
                    if let (Some(path), Ok(..)) = (
                        dir_entry.file_name().to_str(),
                        dir_entry.metadata(),
                    ) {
                        let mut filedata = BTreeMap::<String, Attribute>::new();

                        filedata.insert("selected".to_string(), 
                        Attribute::Literal(Value::Bool(false)));

                        filedata.insert("filepath".to_string(),
                        Attribute::Literal(Value::TextBuffer(dir_entry.path().to_string_lossy().to_string())));

                        map.insert(
                           path.to_string(),
                           Attribute::Map(filedata),
                        );
                    }
                }
            }
            Some(Attribute::Map(map))
        } else {
        None
    }
}
