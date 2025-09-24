use todo::model::value_object::Todo;

#[test]
fn test_parse() {
    let json = r#"{"title": "Buy groceries", "completed": false}"#;
    let todo = Todo::parse(json);

    assert_eq!(todo.id, 0);
    assert_eq!(todo.title, "Buy groceries");
    assert_eq!(todo.completed, false);
}

#[test]
fn test_parse_id() {
    let json = r#"{"id": 1, "title": "Buy groceries", "completed": false}"#;
    let todo = Todo::parse(json);

    assert_eq!(todo.id, 1);
    assert_eq!(todo.title, "Buy groceries");
    assert_eq!(todo.completed, false);
}

#[test]
fn test_parse_title() {
    let json = r#"{"title": "Buy groceries"}"#;
    let todo = Todo::parse(json);

    assert_eq!(todo.id, 0);
    assert_eq!(todo.title, "Buy groceries");
    assert_eq!(todo.completed, false);
}

#[test]
fn test_parse_completed() {
    let json = r#"{"completed": true}"#;
    let todo = Todo::parse(json);

    assert_eq!(todo.id, 0);
    assert_eq!(todo.title, "");
    assert_eq!(todo.completed, true);
}

#[test]
fn test_parse_array() {
    let json = r#"[{"title": "Buy groceries", "completed": false}]"#;
    let todos = Todo::parse_array(json);

    assert_eq!(todos.len(), 1);

    assert_eq!(todos[0].id, 0);
    assert_eq!(todos[0].title, "Buy groceries");
    assert_eq!(todos[0].completed, false);
}

#[test]
fn test_parse_array_multiple() {
    let json = r#"[
        {"title": "Buy groceries", "completed": false},
        {"title": "Water the plants", "completed": true}
    ]"#;

    let todos = Todo::parse_array(json);

    assert_eq!(todos.len(), 2);

    assert_eq!(todos[0].id, 0);
    assert_eq!(todos[0].title, "Buy groceries");
    assert_eq!(todos[0].completed, false);

    assert_eq!(todos[1].id, 0);
    assert_eq!(todos[1].title, "Water the plants");
    assert_eq!(todos[1].completed, true);
}

#[test]
fn test_parse_array_empty() {
    let json = r#"[]"#;
    let todos = Todo::parse_array(json);

    assert_eq!(todos.len(), 0);
}

#[test]
fn test_from_array_to_json() {
    let json = r#"[
    {"id": 0, "title": "abc", "completed": false}
]"#;

    let todos = [Todo{ id: 0, title: "abc".to_string(), completed: false }];

    assert_eq!(json, Todo::stringify_array(&todos));
}
