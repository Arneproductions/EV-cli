use std::{fs, path};

/// Searches for terminal configurations and lists the available terminals
pub fn get_terminals() -> Vec<String> {

    let path = path::Path::new("");
    fs::read_dir(path);
    return Vec::new();
}