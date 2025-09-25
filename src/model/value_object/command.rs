use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Command {
    pub subcommand: (String, String),
    pub options: HashMap<String, String>,
    pub extra_args: Vec<String>,
    pub result: Result<String, String>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            subcommand: ("".to_string(), "".to_string()),
            options: HashMap::new(),
            extra_args: Vec::new(),
            result: Ok(String::new()),
        }
    }
}
