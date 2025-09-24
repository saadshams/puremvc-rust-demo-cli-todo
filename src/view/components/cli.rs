use std::any::Any;
use std::sync::{Arc};
use crate::model::value_object::{Command, Todo};

pub struct CLI {
    delegate: Option<Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync>>
}

impl CLI {
    pub fn new() -> Self {
        CLI { delegate: None }
    }

    pub fn parse(&self, args: Vec<String>) {
        if args.len() < 2 { return eprintln!("Usage: todo <command> [options]"); }

        let mut i = 1; // skip program name

        let mut command = Command::new();

        // Subcommand
        if i < args.len() && !args[i].starts_with('-') && !args[i].starts_with("--") {
            command.subcommand.0 = args[i].clone();
            i += 1;
        }

        // Optional subcommand argument
        if i < args.len() && !args[i].starts_with('-') {
            command.subcommand.1 = args[i].clone();
            i += 1;
        }

        // Parse remaining options
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") || arg.starts_with('-') {
                let key = arg.clone();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') && !args[i + 1].starts_with("--") {
                    command.options.insert(key, args[i + 1].clone()); // key + value
                    i += 2;
                } else {
                    command.options.insert(key, "true".to_string()); // key
                    i += 1;
                }
            } else {
                // extra positional argument
                i += 1;
            }
        }

        if let Some(delegate) = &self.delegate {
            delegate(Arc::new(command));
        }
    }

    pub fn result(&self, todos: &[Todo]) {
        println!("{}", Todo::print_array(todos));
    }

    pub fn fault(&self, error: String) {
        println!("Fault occurred! {}", error);
    }

    pub fn set_delegate(&mut self, delegate: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync>) {
        self.delegate = Some(delegate);
    }
}

/*
[
    {"id": 1, "title": "Buy groceries", "completed": false},
    {"id": 2, "title": "Water the plants", "completed": false},
    {"id": 3, "title": "Pay bills", "completed": false},
    {"id": 4, "title": "Finish project report", "completed": true},
    {"id": 5, "title": "Watch a movie", "completed": true}
]
 */