use std::any::Any;
use std::sync::{Arc};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::SimpleCommand;
use crate::ApplicationFacade;
use crate::model::CLIProxy;
use crate::model::value_object::{Command};

pub struct CLICommand {
    command: SimpleCommand
}

impl CLICommand {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }

    fn result(&self, result: Result<Command, String>) {
        match result {
            Ok(command) => self.send_notification(ApplicationFacade::CLI_RESULT, Some(Arc::new(command)), None),
            Err(error) => self.send_notification(ApplicationFacade::CLI_FAULT, Some(Arc::new(error)), None),
        }
    }
}

impl ICommand for CLICommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        let command = notification.body().and_then(|body| body.downcast_ref::<Command>())
            .expect("Notification body must be a Command.")
            .clone(); // now owned

        let proxy_arc = self.facade().retrieve_proxy(CLIProxy::NAME)
            .expect("CLIProxy must exist.");

        let mut guard = proxy_arc.write().unwrap();
        let proxy = guard.as_any().downcast_ref::<CLIProxy>()
            .expect("Proxy must be a CLIProxy.");

        match command.subcommand.0.as_str() {
            "list" => self.result(proxy.list(command)), // found &Command error
            "add" => self.result(proxy.add(command)),
            "edit" => self.result(proxy.edit(command)),
            "delete" => self.result(proxy.delete(command)),
            // "help" => self.result(proxy.help(command)),
            _ => {
                if command.subcommand.0.is_empty() {
                    eprintln!("{}error:{} No subcommand was provided. \n\tSee 'todo --help' for a list of commands.", "\x1b[31;1m", "\x1b[0m");
                } else {
                    eprintln!("\x1b[31;1merror:\x1b[0m unrecognized command '{}'. See 'todo --help'.", command.subcommand.0);
                }
            }
        }
    }
}

impl INotifier for CLICommand {
    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}

/*
if let Some(result) = &command.result {
    if let Some(todos) = result.downcast_ref::<Result<Vec<Todo>, String>>() {
        match todos {
            Ok(list) => println!("Todos: {:?}", list),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
 */