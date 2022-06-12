pub trait AddCommand {
    fn add_variable(&self, var_name: String, value: String) -> ();
}

pub trait ListCommand {
    fn list_variables(&self) -> [String];
    fn list_terminals(&self) -> [String];
}

pub trait RemoveCommand {
    fn remove_variable(&self, var_name: String) -> ();
}