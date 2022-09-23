use std::{fs, collections::HashMap};
use regex::Regex;
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

        let path = if self.use_global { self.global_path.to_string() } else { self.user_path.to_string() };

        let content = fs::read_to_string(path)
            .expect("Could not read the environment configuration file!");

        return content;
    }

    fn parse_environment_variables(&self, content: String) -> HashMap<String, String> {

        /* What we are going to do is that we go through them line by line. For each line we do the following:
        - Try to see if the string matches the regex for a EV
            - If it does then we try and split the string with ':' 
                - Add it/them to the hashmap. If it already exists then overwrite the value
            - If not we add it as an unknown line that cannot be seen or manipulated with the interface provided
        */
        // Create regex
        // export (?'name'\w+)=\"?((?'value'(.+):?))(\"?)
        // Parse file with regeg
        
        // Add them in a hashmap

        return HashMap::new();
    }
    fn save_environment_variables(&self, variables: HashMap<String, String>) {
        // Build string of variables
        // Write to file
    }
}

impl RemoveCommand for ZSHHandler {
    fn remove_variable(&self, var_name: String) -> () {

        let content = self.read_file();
        let mut variables = self.parse_environment_variables(content);
        
        // Remove the environment variable
        variables.remove(&var_name);
        
        self.save_environment_variables(variables);
        return;
    }
}

impl AddCommand for ZSHHandler {
    fn add_variable(&self, var_name: String, value: String, overwrite: bool) -> () {
        
        let content = self.read_file();
        let mut variables = self.parse_environment_variables(content);

        // If we are not allowed to overwrite a variable and it exists then terminate
        if(!overwrite && variables.contains_key(&var_name)) {
            return;
        }

        variables.insert(var_name, value);
        self.save_environment_variables(variables);

        return;
    }
}

impl ListCommand for ZSHHandler {
    fn list_variables(&self, filter: String) -> Vec<String> {

        let content = self.read_file();
        
        return Vec::new();
    }

    fn list_terminals(&self) -> Vec<String> {

        let terminals = super::get_terminals();
        return terminals;
    }
}

impl TerminalHandler for ZSHHandler {

}