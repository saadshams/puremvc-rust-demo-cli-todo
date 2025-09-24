use std::collections::HashMap;
use crate::model::value_object::Todo;

#[derive(Clone)] 
pub struct Command {
    pub subcommand: (String, String),
    pub options: HashMap<String, String>,
    pub result: Option<Result<Vec<Todo>, String>>,
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
