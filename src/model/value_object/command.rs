use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Command {
    pub subcommand: (String, String),
    pub options: HashMap<String, String>,
    pub result: Result<String, String>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            subcommand: ("".to_string(), "".to_string()),
            options: HashMap::new(),
            result: Ok(String::new()),
        }
    }
}
