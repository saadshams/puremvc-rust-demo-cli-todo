use std::any::Any;
use std::sync::{Arc};
use crate::model::value_object::{Command};

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
        if i < args.len() {
            let arg = &args[i];
            if !arg.is_empty() {
                command.subcommand.0 = arg.clone();
                i += 1;
            }
        }

        // Subcommand argument
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
                i += 1; // extra positional argument
            }
        }

        if let Some(delegate) = &self.delegate {
            delegate(Arc::new(command));
        }
    }

    pub fn result(&self, command: Command) {
        if let Some(result) = command.result {
            match result {
                Ok(body) => println!("{}", &body),
                Err(error) => eprintln!("\x1b[31;1merror:\x1b[0m {}", error),
            }
        } else {
            eprintln!("\x1b[31;1merror:\x1b[0m No valid result in command.");
        }
    }

    pub fn fault(&self, error: String) {
        println!("{}", error);
    }

    pub fn set_delegate(&mut self, delegate: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync>) {
        self.delegate = Some(delegate);
    }
}
