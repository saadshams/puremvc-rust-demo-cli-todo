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
        if args.len() < 2 {
            eprintln!("Usage: todo <command> [options]");
            return;
        }

        let mut i = 1; // skip program name
        let mut command = Command::new();

        // Subcommand name
        if i < args.len() && !args[i].starts_with('-') {
            command.subcommand.0 = args[i].clone();
            i += 1;
        }

        // Subcommand argument
        if i < args.len() && !args[i].starts_with('-') {
            command.subcommand.1 = args[i].clone();
            i += 1;
        }

        // Options
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("-") {
                let key = arg.clone();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    command.options.insert(key, args[i + 1].clone()); // key + value
                    i += 2;
                } else {
                    command.options.insert(key, "true".to_string()); // key
                    i += 1;
                }
            } else {
                command.extra_args.push(arg.clone()); // extra arguments
                i += 1;
            }
        }

        if let Some(delegate) = &self.delegate {
            delegate(Arc::new(command));
        }
    }

    pub fn result(&self, command: Command) {
        println!("{}", command.result.unwrap());
    }

    pub fn fault(&self, error: String) {
        println!("{}", error);
    }

    pub fn set_delegate(&mut self, delegate: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync>) {
        self.delegate = Some(delegate);
    }
}
