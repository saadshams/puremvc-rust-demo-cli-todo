use std::any::Any;
use std::sync::{Arc};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::SimpleCommand;
use crate::ApplicationFacade;
use crate::model::CLIProxy;
use crate::model::value_object::Command;

pub struct CLICommand {
    command: SimpleCommand
}

impl CLICommand {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl ICommand for CLICommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<Command>())
            .map(|command| {
                let arc = self.facade().retrieve_proxy(CLIProxy::NAME).unwrap();
                if let Some(proxy) = arc.write().unwrap().as_any().downcast_ref::<CLIProxy>() {
                    match command.subcommand.0.as_str() {
                        "list" => {
                            let todos = Arc::new(proxy.list());
                            self.send_notification(ApplicationFacade::CLI_RESULT, Some(todos), None);
                        }
                        "add" => {
                            match proxy.add(&command) {
                                Ok(todos) => {
                                    self.send_notification(ApplicationFacade::CLI_RESULT, Some(Arc::new(todos)), None);
                                },
                                Err(error) => {
                                    self.send_notification(ApplicationFacade::CLI_FAULT, Some(Arc::new(error)), None);
                                },
                            }
                        }
                        "edit" => {
                            match proxy.edit(&command) {
                                Ok(todos) => {
                                    self.send_notification(ApplicationFacade::CLI_RESULT, Some(Arc::new(todos)), None);
                                },
                                Err(error) => {
                                    self.send_notification(ApplicationFacade::CLI_FAULT, Some(Arc::new(error)), None);
                                }
                            }
                        }
                        "delete" => {
                            match proxy.delete(command) {
                                Ok(todos) => {
                                    self.send_notification(ApplicationFacade::CLI_RESULT, Some(Arc::new(todos)), None);
                                }
                                Err(error) => {
                                    self.send_notification(ApplicationFacade::CLI_FAULT, Some(Arc::new(error)), None);
                                }
                            }

                        }
                        _ => {
                            if command.subcommand.0.is_empty() {
                                eprintln!("{}error:{} No subcommand was provided. \n\tSee 'todo --help' for a list of commands.", "\x1b[31;1m", "\x1b[0m");
                            } else {
                                eprintln!("{}error:{} unrecognized command '{}'. See 'todo --help'.", "\x1b[31;1m", "\x1b[0m", command.subcommand.0);
                            }
                        }
                    }
                }
            });
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
