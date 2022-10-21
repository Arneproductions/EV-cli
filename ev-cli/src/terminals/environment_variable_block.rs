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
    pub fn new() -> Self {

        Self {
            key: String::new(),
            value : String::new(),
            misc: Vec::new()
        }
    }

    pub fn add_misc(&mut self, line: &str) {

        self.misc.push(line.to_string());
    }

    pub fn set_environment_variable(&mut self, key: &str, value: &str) {

        self.key = key.to_owned();
        self.value = value.to_owned();
    }

    pub fn to_string(&self) -> String {

        let mut value = String::new();

        for line in self.misc.iter() {
            value.push_str(line.as_str());
        }

        let ev_line = format!("export {}={}", self.key, self.value);

        return ev_line;
    }
}