use std::{fs, collections::HashMap, path::{PathBuf}};
use regex::Regex;
use crate::{cmd::command_handlers::{RemoveCommand, AddCommand, ListCommand}, io};
use super::{TerminalHandler, EnvironmentVariableBlock};

const USER_PATH: &str = ".zshrc";
const GLOBAL_PATH: &str = "/etc/zprofile";
const ENV_SCRIPT_TAG: &str = "EVC_SCRIPT_LINES";

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

    fn get_file_path(&self) -> PathBuf {

        fn build_path(mut home_path: PathBuf, filename: &str) -> PathBuf {
            home_path.push(filename);

            return home_path;
        }

        let path = if self.use_global { build_path(PathBuf::new(), self.global_path.as_str()) } else { 
            match io::get_home_dir() {
                Some(home_path) => build_path(home_path, self.user_path.as_str()),
                None => PathBuf::new()
            }
        };

        return path;
    }
    
    fn read_file(&self) -> String {

        let path = self.get_file_path();

        let content = fs::read_to_string(path)
            .expect("Could not read the environment configuration file!");

        return content;
    }

    fn parse_environment_variables(&self, content: String) -> HashMap<String, EnvironmentVariableBlock> {

        // TODO: Fix regex to be RUST compatible
        let env_var_regex = Regex::new("^export (?P<name>\\w+)=(?P<value>[a-z,A-Z,<>-_`¨^~'./,:;]+)$").unwrap();
        let mut environment_variables = HashMap::new();

        let mut ev_block = EnvironmentVariableBlock::new();

        for line in content.split("\n") {
        
            if env_var_regex.is_match(line) {

               let captures = env_var_regex.captures(line).unwrap();
               
               let name = captures.name("Name").expect("Environment variable is missing a name");
               let value = captures.name("Value").expect("Environment variable is missing a value");
               ev_block.set_environment_variable(name.as_str(), value.as_str());

               environment_variables.insert(name.as_str().to_owned(), ev_block);
               ev_block = EnvironmentVariableBlock::new();
            } else {
                ev_block.add_misc(line);
            } 
        }

        // Make sure to include strings that is not export VAR=VALUE. This could be scripts
        environment_variables.insert(ENV_SCRIPT_TAG.to_string(), ev_block);

        return environment_variables;
    }

    fn save_environment_variables(&self, variables: HashMap<String, EnvironmentVariableBlock>) {
        // Build string of variables

        let mut sb: String = "".to_owned();

        for (_, value) in variables {

            sb.push_str(&value.to_string());
        }

        // Write the changes to the file
        let path = self.get_file_path();
        fs::write(path, sb).expect("Could save state to file...");
    }
}

impl RemoveCommand for ZSHHandler {
    fn remove_variable(&self, var_name: &str) -> () {

        let content = self.read_file();
        let mut variables = self.parse_environment_variables(content);
        
        // Remove the environment variable
        variables.remove(var_name);
        
        self.save_environment_variables(variables);
        return;
    }
}

impl AddCommand for ZSHHandler {
    fn add_variable(&self, var_name: &str, value: &str, overwrite: &bool) -> () {
        
        let content = self.read_file();
        let mut variables = self.parse_environment_variables(content);

        if ENV_SCRIPT_TAG == var_name {
            return; // DO NOT EVEN... you bastard
        }

        // If we are not allowed to overwrite a variable and it exists then terminate
        if !overwrite && variables.contains_key(var_name) {
            return;
        }

        let mut ev_block = EnvironmentVariableBlock::new();
        ev_block.set_environment_variable(var_name, value);

        variables.insert(var_name.to_string(), ev_block);
        self.save_environment_variables(variables);

        return;
    }
}

impl ListCommand for ZSHHandler {
    fn list_variables(&self, filter: &str) -> Vec<String> {

        let content = self.read_file();
        let variables = self.parse_environment_variables(content);

        let mut names = Vec::new();
        for (name, _) in variables {
            
            if name.contains(&filter) && ENV_SCRIPT_TAG != name {

                names.push(name);
            }
        }
        
        return names;
    }

    fn list_terminals(&self) -> Vec<String> {

        let terminals = super::get_terminals();
        return Vec::from_iter(terminals);
    }
}

impl TerminalHandler for ZSHHandler {

}