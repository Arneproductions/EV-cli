pub trait AddCommand {
    fn add_variable(&self, var_name: &str, value: &str, overwrite: &bool) -> ();
}

pub trait ListCommand {
    fn list_variables(&self, filter: &str) -> Vec<String>;
    fn list_terminals(&self) -> Vec<String>;
}

pub trait RemoveCommand {
    fn remove_variable(&self, var_name: &str) -> ();
}