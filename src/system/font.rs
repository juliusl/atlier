

pub fn cascadia_code() -> Option<Vec<u8>>{
    if let Ok(home) = std::env::var("HOME") {
        let path =&format!("{}/Library/Fonts/CascadiaCode.ttf", home);
        if let Ok(r) = std::fs::read(std::path::Path::new(path)) {
            Some(r)
        } else if let Ok(r) = std::fs::read(std::path::Path::new("C:/Windows/Fonts/CascadiaCode.ttf")) {
            Some(r)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn monaco() -> Option<Vec<u8>>{
    if let Ok(r) = std::fs::read(std::path::Path::new("/System/Library/Fonts/Monaco.ttf")) {
        Some(r)
    } else if let Ok(r) = std::fs::read(std::path::Path::new("C:/Windows/Fonts/Monaco.ttf")) {
        Some(r)
    } else {
        None
    }
}