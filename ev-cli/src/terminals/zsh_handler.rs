use std::{fs};

use crate::cmd::command_handlers::{RemoveCommand, AddCommand, ListCommand};

use super::TerminalHandler;

const USER_PATH: String = String::from("/sdf");
const GLOBAL_PATH: String = String::from("bla bla");

pub struct ZSHHandler {

    user_path: String,
    global_path: String,
}

impl ZSHHandler {
    pub fn new() -> Self {
        Self { 
            user_path: USER_PATH, 
            global_path: GLOBAL_PATH 
        }
    }

    fn read_file(&self, path: &String) -> String {
        let content = fs::read_to_string(path)
            .expect("Could not read the environment configuration file!");

        return content;
    }
}

impl RemoveCommand for ZSHHandler {
    fn remove_variable(&self, var_name: String) -> () {

        let content = self.read_file(&self.user_path);
        return;
    }
}

impl AddCommand for ZSHHandler {
    fn add_variable(&self, var_name: String, value: String) -> () {
        
        let content = self.read_file(&self.user_path);

        return;
    }
}

impl ListCommand for ZSHHandler {
    fn list_variables(&self) -> Vec<String> {

        let content = self.read_file(&self.user_path);
        
        return Vec::new();
    }

    fn list_terminals(&self) -> Vec<String> {

        let content = self.read_file(&self.user_path);

        return Vec::new();
    }
}

impl TerminalHandler for ZSHHandler {

}