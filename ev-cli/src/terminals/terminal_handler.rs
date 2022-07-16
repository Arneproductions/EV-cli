use crate::cmd::command_handlers::{AddCommand, RemoveCommand, ListCommand};



pub trait TerminalHandler: AddCommand + RemoveCommand + ListCommand {

}