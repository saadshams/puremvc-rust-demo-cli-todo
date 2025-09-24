use std::any::Any;
use std::sync::Arc;
use todo::model::value_object::Command;
use todo::view::components::CLI;

#[test]
fn test_parse_add() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "add");
                assert_eq!(cmd.subcommand.1, "Buy groceries");
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "add".into(), "Buy groceries".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_add_active() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "add");
                assert_eq!(cmd.subcommand.1, "Buy groceries");
                assert_eq!(cmd.options.get("--active"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "add".into(), "Buy groceries".into(),
        "--active".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_add_completed() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "add");
                assert_eq!(cmd.subcommand.1, "Buy groceries");
                assert_eq!(cmd.options.get("--completed"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "add".into(), "Buy groceries".into(),
        "--completed".into(),
    ];

    cli.parse(command);
}
