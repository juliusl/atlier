use crate::prelude::*;

pub struct ListDirectory;

impl NodeExterior for ListDirectory {
    fn title() -> &'static str {
        "List Directory"
    }

    fn group_name() -> &'static str {
        "Filesystem"
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

    fn parameter() -> NodeResource {
        NodeResource::Attribute(
            Self::param_name,
            Self::input,
            Some(Attribute::Literal(Value::TextBuffer("./".to_string()))),
            None,
        )
    }
}

fn read_dir(path: &str) -> Option<Attribute> {
    if let Ok(paths) = std::fs::read_dir(path) {
            let mut map = State::default();
            for path in paths {
                if let Ok(dir_entry) = path {
                    if let (Some(path), Ok(..)) = (
                        dir_entry.file_name().to_str(),
                        dir_entry.metadata(),
                    ) {
                        let filedata = State::default()
                            .insert("selected", false)
                            .insert("filepath", dir_entry.path().to_string_lossy().to_string());

                        map = map.insert(path, filedata);
                    }
                }
            }
            Some(map.into())
        } else {
        None
    }
}
