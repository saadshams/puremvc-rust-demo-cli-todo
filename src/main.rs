use std::env;
use std::sync::{Arc, RwLock};
use crate::view::components::CLI;

mod controller;
mod model;
mod view;
mod application_facade;

pub use application_facade::ApplicationFacade;

fn main() {
    let cli = Arc::new(RwLock::new(CLI::new()));

    // Startup facade and register commands, mediators and delegates
    ApplicationFacade::get_instance("Todo").startup(cli.clone());

    // Defer `parse()`, until after facade setup
    // Ensures notifications are sent only after mediators and delegates are registered
    cli.read().unwrap().parse(env::args().collect()); // `parse()` will trigger notifications
}
