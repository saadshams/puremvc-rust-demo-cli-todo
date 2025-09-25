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
            Ok(command) =>
                self.send_notification(ApplicationFacade::CLI_RESULT, Some(Arc::new(command)), None),
            Err(error) =>
                self.send_notification(ApplicationFacade::CLI_FAULT, Some(Arc::new(error)), None),
        }
    }
}

impl ICommand for CLICommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        let proxy_arc = self.facade().retrieve_proxy(CLIProxy::NAME)
            .expect("[CLICommand] Error: CLIProxy must exist.");

        let mut guard = proxy_arc.write().unwrap();
        let proxy = guard.as_any().downcast_ref::<CLIProxy>()
            .expect("[CLICommand] Error: Proxy must be a CLIProxy.");

        let command = notification.body().and_then(|body| body.downcast_ref::<Command>())
            .expect("[CLICommand] Error: Notification body must be a Command.")
            .clone();

        match command.subcommand.0.as_str() {
            "list" => self.result(proxy.list(command)),
            "add" => self.result(proxy.add(command)),
            "edit" => self.result(proxy.edit(command)),
            "delete" => self.result(proxy.delete(command)),
            _ => {
                if command.options.contains_key("-h") || command.options.contains_key("--help") {
                    self.result(proxy.help(command));
                } else if command.options.contains_key("-v") || command.options.contains_key("--version") {
                    self.result(proxy.version(command));
                } else if command.options.contains_key("-r") || command.options.contains_key("--reset") {
                    self.result(proxy.reset(command));
                } else if command.subcommand.0.is_empty() {
                    self.result(Err("\x1b[31;1mError:\x1b[0m No Command or Subcommand was provided. See 'todo --help' for a list of commands.".to_string()));
                } else {
                    self.result(Err(format!("\x1b[31;1mError:\x1b[0m Unrecognized command '{}'. See 'todo --help'.", command.subcommand.0)));
                };
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
