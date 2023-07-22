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
        // TODO: Fix so that the misc lines is being added as well
        let mut value = String::new();

        for line in self.misc.iter() {
            value.push_str(line.as_str());
        }

        let ev_line = format!("export {}={}", self.key, self.value);

        return format!("{}{}\n", value, ev_line);
    }
}

#[cfg(test)]
mod environment_variable_block_tests {
    use super::EnvironmentVariableBlock;


    #[test]
    fn to_string_returns_misc_lines_and_ev_line() {

        let mut ev_block = EnvironmentVariableBlock::new();

        ev_block.add_misc("#This is a comment\n");
        ev_block.add_misc("#This is another comment that we also need to take care of\n");
        ev_block.set_environment_variable("key", "value");

        let result = ev_block.to_string();
        assert_eq!(result, "#This is a comment\n#This is another comment that we also need to take care of\nexport key=value\n")
    }
}