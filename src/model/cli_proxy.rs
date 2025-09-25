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
    pub const NAME: &'static str = "CLIProxy";

    pub fn new() -> Self {
        Self {
            proxy: Proxy::new(Some(Self::NAME), None),
        }
    }

    fn read(&self) -> Result<String, String> {
        fs::read_to_string("todos.json")
            .map_err(|e| format!("\x1b[31;1mError:\x1b[0m Unable to read file 'todos.json': {}", e))
    }

    fn write(&self, json: &str) -> Result<(), String> {
        fs::write("todos.json", format!("{}", json))
            .map_err(|error| format!("\x1b[31;1mError:\x1b[0m Unable to write file 'todos.json': {}", error))
    }

    pub fn list(&self, mut command: Command) -> Result<Command, String> {
        let todos = Todo::parse_array(&self.read()?);
        command.result = Ok(Todo::print_array(&todos));
        Ok(command)
    }

    pub fn add(&self, mut command: Command) -> Result<Command, String> {
        let mut todos = Todo::parse_array(&self.read()?);

        if let Some(title) = command.options.get("-t").or_else(|| command.options.get("--title")) {
            let id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
            todos.push(Todo { id, title: title.clone(), completed: false });

            self.write(&Todo::stringify_array(&todos))?;

            command.result = Ok(Todo::print_array(&todos));
            Ok(command)
        } else {
            Err("\x1b[31;1mError:\x1b[0m Missing required argument: --title (-t)".into())
        }
    }

    pub fn edit(&self, mut command: Command) -> Result<Command, String> {
        let title = command.options.get("-t").or_else(|| command.options.get("--title"))
            .ok_or_else(|| "\x1b[31;1mError:\x1b[0m Missing required argument: --title (-t)".to_string())?
            .clone();

        let completed = command.options.get("-c").or_else(|| command.options.get("--completed"))
            .map(|s| matches!(s.as_str(), "true"))
            .unwrap_or(false);

        let id = command.subcommand.1.parse::<u32>()
            .map_err(|e| format!("\x1b[31;1mError:\x1b[0m Invalid ID format: {e}"))?;

        let mut todos = Todo::parse_array(&self.read()?);

        for todo in todos.iter_mut() {
            if todo.id == id {
                todo.title = title.clone();
                todo.completed = completed;
            }
        }

        self.write(&Todo::stringify_array(&todos))?;

        command.result = Ok(Todo::print_array(&todos));
        Ok(command)
    }

    pub fn delete(&self, mut command: Command) -> Result<Command, String> {
        let id = command.subcommand.1
            .parse::<u32>()
            .map_err(|e| format!("\x1b[31;1mError:\x1b[0m Invalid ID format: {e}"))?;

        let mut todos = Todo::parse_array(&self.read()?);
        todos.retain(|todo| todo.id != id);
        for (i, todo) in todos.iter_mut().enumerate() {
            todo.id = (i + 1) as u32;
        }

        self.write(&Todo::stringify_array(&todos))?;

        command.result = Ok(Todo::print_array(&todos));
        Ok(command)
    }

    pub fn help(&self, mut command: Command) -> Result<Command, String> {
        let body = String::from(
            "Available commands:\n\
         - list                  List all todos\n\
         - add -t <title>        Add a new todo\n\
         - edit <id> -t <title>  Edit a todo\n\
         - delete <id>           Delete a todo\n\
         - help                  Show this help message\n\
         - version               Show the version\n\
         - reset                 Reset todos.json to default"
        );

        command.result = Ok(body);
        Ok(command)
    }

    pub fn version(&self, mut command: Command) -> Result<Command, String> {
        let body = format!("todo version {}", env!("CARGO_PKG_VERSION"));

        command.result = Ok(body);
        Ok(command)
    }

    pub fn reset(&self, mut command: Command) -> Result<Command, String> {
        self.write(r#"[
    {"id": 1, "title": "Buy groceries", "completed": false},
    {"id": 2, "title": "Water the plants", "completed": false},
    {"id": 3, "title": "Read a book", "completed": true},
    {"id": 4, "title": "Write a report", "completed": false},
    {"id": 5, "title": "Watch a movie", "completed": true}
]"#)?;

        let todos = Todo::parse_array(&self.read()?);
        command.result = Ok(Todo::print_array(&todos));
        Ok(command)
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
