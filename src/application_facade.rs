use std::any::Any;
use std::sync::{Arc};
use puremvc::interfaces::{IFacade, INotifier};
use puremvc::patterns::Facade;
use crate::controller::StartupCommand;

pub struct ApplicationFacade {
    facade: Arc<dyn IFacade>
}

impl ApplicationFacade {
    pub const STARTUP: &'static str = "startup";
    pub const CLI: &'static str = "cli";
    pub const CLI_RESULT: &'static str = "cliResult";
    pub const CLI_FAULT: &'static str = "cliFault";

    pub fn new(key: &str) -> Self {
        Self {
            facade: Facade::get_instance(key, |k| Facade::new(k))
        }
    }

    pub fn initialize_controller(&self) {
        self.facade.initialize_controller();
        self.facade.register_command(Self::STARTUP, || Box::new(StartupCommand::new()));
    }

    pub fn get_instance(key: &str) -> Arc<ApplicationFacade> {
        let instance = Arc::new(ApplicationFacade::new(key));
        instance.initialize_controller();
        instance
    }

    pub fn startup(&self, component: Arc<dyn Any + Send + Sync>) {
        self.facade.send_notification(Self::STARTUP, Some(component), None);
    }
}

impl INotifier for ApplicationFacade {
    fn facade(&self) -> Arc<dyn IFacade> {
        self.facade.clone()
    }

    fn initialize_notifier(&mut self, _key: &str) {
        
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.facade.send_notification(name, body, type_);
    }
}

impl IFacade for ApplicationFacade {}
