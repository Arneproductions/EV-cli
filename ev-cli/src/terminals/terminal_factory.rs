use super::{TerminalHandler, ZSHHandler};



pub fn create_terminal(use_global: bool) -> Box<dyn TerminalHandler> {

    return Box::new(ZSHHandler::new(use_global));
}