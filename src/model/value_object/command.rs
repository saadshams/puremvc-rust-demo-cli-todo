use std::collections::HashMap;

#[derive(Clone)]
pub struct Command {
    pub subcommand: (String, String),
    pub options: HashMap<String, String>,
    pub result: Option<Result<String, String>>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            subcommand: ("".to_string(), "".to_string()),
            options: HashMap::new(),
            result: None,
        }
    }
}
