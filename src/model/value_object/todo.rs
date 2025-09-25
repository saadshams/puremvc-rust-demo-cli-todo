#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool
}

impl Todo {
    pub fn stringify(&self) -> String {
        format!(r#"{{"id": {}, "title": "{}", "completed": {}}}"#,
            self.id, self.title.replace('\\', "\\\\").replace('"', "\\\""), self.completed
        )
    }

    pub fn stringify_array(todos: &[Todo]) -> String {
        let items = todos.iter()
            .map(|todo| format!("    {}", todo.stringify()))
            .collect::<Vec<String>>()
            .join(",\n");
        
        format!("[\n{}\n]", items)
    }

    pub fn parse(json: &str) -> Todo {
        let mut todo = Todo{id: 0, title: String::new(), completed: false};

        for pair in json.trim().trim_matches(|c| c == '{' || c == '}').split(',') {
            let mut property = pair.trim().splitn(2, ':');

            let key = property.next()
                .map(|k| k.trim().trim_matches('"'))
                .unwrap_or("");

            let value = property.next()
                .map(|v| v.trim())
                .unwrap_or("");

            match key {
                "id" => todo.id = value.parse::<u32>().unwrap_or(0),
                "title" => todo.title = value.trim_matches('"').to_string(),
                "completed" => todo.completed = value.parse::<bool>().unwrap_or(false),
                _ => {}
            }
        }

        todo
    }

    pub fn parse_array(json: &str) -> Vec<Todo> {
        let body = json.trim().strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(json);

        body.split("},")
            .filter_map(|chunk| {
                let chunk = chunk.trim();
                if chunk.is_empty() { None } else { Some(Todo::parse(chunk)) }
            })
            .collect()
    }

    pub fn print(todo: &Todo) -> String {
        format!(
            "{{\x1b[34m\"id\"\x1b[0m: \x1b[35m{}\x1b[0m, \
            \x1b[34m\"title\"\x1b[0m: \x1b[32m\"{}\"\x1b[0m, \
            \x1b[34m\"completed\"\x1b[0m: \x1b[38;5;208m{}\x1b[0m}}",
            todo.id,
            todo.title.replace('\\', "\\\\").replace('"', "\\\""),
            todo.completed
        )
    }

    pub fn print_array(todos: &[Todo]) -> String {
        let body = todos.iter()
            .map(|todo| format!("  {}", Todo::print(todo)))
            .collect::<Vec<String>>()
            .join(",\n");

        format!("[\n{}\n]", body)
    }
}
