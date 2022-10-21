mod terminal_handler;
mod zsh_handler;
mod terminal_factory;
mod terminal_utils;
mod environment_variable_block;

use environment_variable_block::EnvironmentVariableBlock;
pub use terminal_handler::TerminalHandler;
pub use zsh_handler::ZSHHandler;
pub use terminal_factory::create_terminal;
pub use terminal_utils::get_terminals;
