use std::{fs, collections::HashMap, format, borrow::Borrow};
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

    fn get_file_path(&self) -> String {

        let path = if self.use_global { self.global_path.to_string() } else { self.user_path.to_string() };
        return path;
    }
    
    fn read_file(&self) -> String {

        let path = self.get_file_path();

        let content = fs::read_to_string(path)
            .expect("Could not read the environment configuration file!");

        return content;
    }

    fn parse_environment_variables(&self, content: String) -> HashMap<String, String> {

        let env_var_regex = Regex::new("^export (?'name'\\w+)=(?'value'[a-z,A-Z,<>-_`¨^~'.,:;\\/]+)$").unwrap();
        let mut environment_variables = HashMap::new();

        for line in content.split("\n") {

            if env_var_regex.is_match(line) {

               let captures = env_var_regex.captures(line).unwrap();
               
               let name = captures.name("Name").expect("Environment variable is missing a name");
               let value = captures.name("Value").expect("Environment variable is missing a value");

               environment_variables.insert(name.as_str().to_owned(), value.as_str().to_owned());
            } 
        }

        return environment_variables;
    }

    fn save_environment_variables(&self, variables: HashMap<String, String>) {
        // Build string of variables

        let mut sb: String = "".to_owned();

        for (name, value) in variables {
           
            sb.push_str(format!("export {}={}\n", name, value).borrow());
        }

        // Write the changes to the file
        let path = self.get_file_path();
        fs::write(path, sb).expect("Could save state to file...");
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
        if !overwrite && variables.contains_key(&var_name) {
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
        let variables = self.parse_environment_variables(content);

        let mut names = Vec::new();
        for (name, _) in variables {
            
            if name.contains(&filter) {

                names.push(name);
            }
        }
        
        return names;
    }

    fn list_terminals(&self) -> Vec<String> {

        let terminals = super::get_terminals();
        return terminals;
    }
}

impl TerminalHandler for ZSHHandler {

}