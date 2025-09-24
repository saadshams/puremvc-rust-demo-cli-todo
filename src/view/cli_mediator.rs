use std::any::{Any};
use std::sync::{Arc, RwLock, Weak};
use puremvc::interfaces::{IFacade, IMediator, INotification, INotifier};
use puremvc::patterns::Mediator;
use crate::model::value_object::{Todo};
use crate::application_facade::ApplicationFacade;
use crate::view::components::CLI;

pub struct CLIMediator {
    mediator: Mediator
}

impl CLIMediator {
    pub const NAME: &'static str = "UtilityMediator";

    pub fn new(component: &Arc<dyn Any + Send + Sync>) -> CLIMediator {
        Self {
            mediator: Mediator::new(Some(Self::NAME), Some(Arc::downgrade(component)))
        }
    }

    fn cli(&self) -> Arc<RwLock<CLI>> {
        self.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<CLI>>().ok())
            .expect("[CLIMediator] Error: Could not initialize CLI component")
    }
}

impl IMediator for CLIMediator {
    fn name(&self) -> &str {
        self.mediator.name()
    }

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![ApplicationFacade::CLI_RESULT.to_string()]
    }

    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        match notification.name() {
            ApplicationFacade::CLI_RESULT => {
                let todos = notification.body()
                    .and_then(|arc| arc.downcast_ref::<Vec<Todo>>().cloned())
                    .expect("[CLIMediator] Error: Could not read todos");
                self.cli().read().unwrap().result(&todos)
            },
            ApplicationFacade::CLI_FAULT => {
                let error = notification.body()
                    .and_then(|arc| arc.downcast_ref::<String>().cloned())
                    .expect("[CLIMediator] Error: Could not read todos");
                self.cli().read().unwrap().fault(error)
            },
            _ => {},
        }
    }

    fn on_register(&mut self) {
        let facade = self.facade();
        let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> =
            Arc::new(move |command: Arc<dyn Any + Send + Sync + 'static>| {
                facade.send_notification(ApplicationFacade::CLI, Some(command), None);
            });

        self.cli().write().unwrap().set_delegate(notify);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl INotifier for CLIMediator {
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}
