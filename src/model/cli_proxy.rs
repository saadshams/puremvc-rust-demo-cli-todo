use std::any::Any;
use std::fs;
use std::sync::Arc;
use puremvc::interfaces::{IFacade, INotifier, IProxy};
use puremvc::patterns::Proxy;
use crate::model::value_object::{Command, Todo};

pub struct CLIProxy {
    pub proxy: Proxy,
}

impl CLIProxy {
    pub const NAME: &'static str = "CLIProx y";

    pub fn new() -> Self {
        Self {
            proxy: Proxy::new(Some(Self::NAME), None),
        }
    }

    pub fn list(&self) -> Vec<Todo> {
        match fs::read_to_string("todos.json") {
            Ok(json) => {
                Todo::parse_array(&json)
            }
            Err(error) => {
                eprintln!("Error reading file: {}", error);
                vec![]
            }
        }
    }

    pub fn add(&self, command: &Command) -> Result<Vec<Todo>, String> {
        let todos = &mut self.list();
        if let Some(title) = command.options.get("-t").or_else(|| command.options.get("--title")) {
            let id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
            todos.push(Todo{id, title: title.clone(), completed: false });

            fs::write("todos.json", format!("{}", Todo::stringify_array(todos)))
                .map_err(|error| format!("Failed to write todos.json: {}", error))?;

            Ok(todos.clone())
        } else {
            Err("No title provided (use -t or --title)".into())
        }
    }

    pub fn edit(&self, command: &Command) -> Result<Vec<Todo>, String> {
        let title = command.options
            .get("-t")
            .or_else(|| command.options.get("--title"))
            .ok_or_else(|| "No title provided (use -t or --title)".to_string())?
            .clone();

        let completed = command.options
            .get("-c")
            .or_else(|| command.options.get("--completed"))
            .map(|s| matches!(s.as_str(), "true"))
            .unwrap_or(false);

        let id = command.subcommand.1
            .parse::<u32>()
            .map_err(|e| format!("Invalid id: {e}"))?;

        let todos = &mut self.list();
        for todo in todos.iter_mut() {
            if todo.id == id {
                todo.title = title.clone();
                todo.completed = completed;
            }
        }

        fs::write("todos.json", format!("{}", Todo::stringify_array(todos)))
            .map_err(|error| format!("Failed to write todos.json: {}", error))?;

        Ok(todos.clone())
    }

    pub fn delete(&self, command: &Command) -> Result<Vec<Todo>, String> {
        let id = command.subcommand.1
            .parse::<u32>()
            .map_err(|e| format!("Invalid id: {e}"))?;

        let todos = &mut self.list();
        todos.retain(|todo| todo.id != id);
        for (i, todo) in todos.iter_mut().enumerate() {
            todo.id = (i + 1) as u32;
        }

        fs::write("todos.json", format!("{}", Todo::stringify_array(todos)))
            .map_err(|error| format!("Failed to write todos.json: {}", error))?;

        Ok(todos.clone())
    }

}

impl IProxy for CLIProxy {
    fn name(&self) -> &str {
        self.proxy.name()
    }

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.proxy.data()
    }

    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.proxy.set_data(data)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl INotifier for CLIProxy {
    fn facade(&self) -> Arc<dyn IFacade> {
        self.proxy.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.proxy.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.proxy.send_notification(name, body, type_);
    }
}

// let file_path = "todos.json";
//
// fs::write("todos.json", r#"[
//     {"title": "Buy groceries", "completed": false},
//     {"title": "Water the plants", "completed": false},
//     {"title": "Read a book", "completed": true},
//     {"title": "Write a report", "completed": false},
//     {"title": "Watch a movie", "completed": true}
// ]"#).expect("Unable to write file");