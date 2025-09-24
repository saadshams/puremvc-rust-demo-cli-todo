use std::collections::HashMap;

#[derive(Debug)]
pub struct Command {
    pub subcommand: (String, String),
    pub options: HashMap<String, String>
}

impl Command {
    pub fn new() -> Self {
        Self {
            subcommand: ("".to_string(), "".to_string()),
            options: HashMap::new()
        }
    }
}