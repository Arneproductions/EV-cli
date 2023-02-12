use std::{fs, path::Path, collections::HashSet};
use home;

/// Searches for terminal configurations and lists the available terminals
pub fn get_terminals() -> HashSet<String> {

    match home::home_dir() {
        Some(path) => return search_terminals(path.as_path()), 
        None => return HashSet::new()    
    }
}

fn search_terminals(path: &Path) -> HashSet<String> {
    
    let mut terminals = HashSet::new();
    let terminal_names = ["zsh", "bash"];
    let dir = fs::read_dir(path).unwrap();

    for file_path in dir {
        
        let p = file_path.unwrap();
        let file = p.file_name().into_string().unwrap();

        for name in terminal_names {
            if file.contains(name) {
                terminals.insert(name.to_owned());
            }
        }
        
    }

    return terminals;
}