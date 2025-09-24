use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::SimpleCommand;
use crate::application_facade::ApplicationFacade;
use crate::controller::CLICommand;
use crate::model::CLIProxy;
use crate::view::CLIMediator;

pub struct StartupCommand {
    command: SimpleCommand
}

impl StartupCommand {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl ICommand for StartupCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(component) = notification.body() {
            self.facade().register_command(ApplicationFacade::CLI, || Box::new(CLICommand::new()));
            self.facade().register_proxy(Arc::new(RwLock::new(CLIProxy::new())));
            self.facade().register_mediator(Arc::new(RwLock::new(CLIMediator::new(component))));
        }
    }
}

impl INotifier for StartupCommand {
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