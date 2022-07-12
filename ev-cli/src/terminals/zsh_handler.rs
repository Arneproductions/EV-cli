use std::{fs, arch::global_asm};

use crate::cmd::command_handlers::{RemoveCommand, AddCommand, ListCommand};

use super::TerminalHandler;

const USER_PATH: &str = "~/.zshrc";
const GLOBAL_PATH: &str = "/etc/zprofile";

pub struct ZSHHandler {

    use_global: bool,
    user_path: String,
    global_path: String,
}

impl ZSHHandler {
    pub fn new(use_global: bool) -> Self {
        Self { 
            use_global: use_global,
            user_path: String::from(USER_PATH), 
            global_path: String::from(GLOBAL_PATH) 
        }
    }

    fn read_file(&self) -> String {

        let path = if self.use_global { self.global_path } else { self.user_path };

        let content = fs::read_to_string(path)
            .expect("Could not read the environment configuration file!");

        return content;
    }
}

impl RemoveCommand for ZSHHandler {
    fn remove_variable(&self, var_name: String) -> () {

        let content = self.read_file();
        return;
    }
}

impl AddCommand for ZSHHandler {
    fn add_variable(&self, var_name: String, value: String) -> () {
        
        let content = self.read_file();

        return;
    }
}

impl ListCommand for ZSHHandler {
    fn list_variables(&self) -> Vec<String> {

        let content = self.read_file();
        
        return Vec::new();
    }

    fn list_terminals(&self) -> Vec<String> {

        let content = self.read_file();

        return Vec::new();
    }
}

impl TerminalHandler for ZSHHandler {

}