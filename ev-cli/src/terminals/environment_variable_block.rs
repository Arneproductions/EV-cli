/*
This struct is for containing everything that is related to something terminal 'script' specific part i.e.
a bunch of comments before an 'export' line or a script block
 */
pub struct EnvironmentVariableBlock {

    key: String,
    value: String,
    misc: Vec<String>,
}

impl EnvironmentVariableBlock {
    fn new() -> Self {

        Self {
            key: String::new(),
            value : String::new(),
            misc: Vec::new()
        }
    }

    fn add_misc(&self, line: &str) {

        self.misc.push(line.to_string());
    }
}