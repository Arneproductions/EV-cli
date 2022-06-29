use crate::cmd::command_handlers::{RemoveCommand, AddCommand, ListCommand};

use super::TerminalHandler;

pub struct ZSHHandler {

    // TODO: These fields should not be public. This was only for learning purposes
    pub user_path: String,
    pub gloabl_path: String,
}

impl RemoveCommand for ZSHHandler {
    fn remove_variable(&self, var_name: String) -> () {
        return;
    }
}

impl AddCommand for ZSHHandler {
    fn add_variable(&self, var_name: String, value: String) -> () {
        // std::File::open("blabla.txt");

        return;
    }
}

impl ListCommand for ZSHHandler {
    fn list_variables(&self) -> Vec<String> {
        return Vec::new();
    }

    fn list_terminals(&self) -> Vec<String> {
        return Vec::new();
    }
}

impl TerminalHandler for ZSHHandler {

}