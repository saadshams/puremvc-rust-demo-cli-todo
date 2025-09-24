use std::any::Any;
use std::sync::Arc;
use todo::model::value_object::Command;
use todo::view::components::CLI;

#[test]
fn test_parse_list() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "list");
                assert_eq!(cmd.subcommand.1, "");
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "list".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_list_all_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "list");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("-a"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "list".into(),
        "-a".into(), "true".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_list_all_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "list");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--all"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "list".into(),
        "--all".into(), "true".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_list_active() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "list");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--active"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "list".into(),
        "--active".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_list_completed() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "list");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--completed"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "list".into(),
        "--completed".into(),
    ];

    cli.parse(command);
}
