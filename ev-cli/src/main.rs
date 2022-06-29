use clap::Parser;

use crate::cmd::command_handlers::ListCommand;

mod cmd;
mod terminals;

fn main() {
    let args = cmd::Args::parse(); // TODO: Use args in some kind of way

    // TODO: The "".toString() is too ugly. Learn a better way
    let handler = terminals::ZSHHandler{user_path : "".to_string(), gloabl_path : "".to_string()};
    
    let terminal_names = handler.list_terminals();

    for t_name in terminal_names {
        println!("{}", t_name);    
    }
}
