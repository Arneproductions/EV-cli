use std::{fs};

/// Searches for terminal configurations and lists the available terminals
pub fn get_terminals() -> Vec<String> {

    let mut terminals = Vec::new();
    let terminal_names = ["zsh", "bash"];
    let dir = fs::read_dir("~").unwrap();
    
    for path in dir {
        
        let p = path.unwrap();
        let file = p.file_name().into_string().unwrap();

        for name in terminal_names {
            if file.contains(name) {
                terminals.push(name.to_owned());
            }
        }
        
    }
    return terminals;
}