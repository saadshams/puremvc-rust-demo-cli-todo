use std::any::Any;
use std::sync::Arc;
use todo::model::value_object::Command;
use todo::view::components::CLI;

#[test]
fn test_parse_delete() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "42");
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(), "42".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_delete_all_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "42");
                assert_eq!(cmd.options.get("-a"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(), "42".into(),
        "-a".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_delete_all_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "42");
                assert_eq!(cmd.options.get("--all"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(), "42".into(),
        "--all".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_delete_all_completed() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--completed"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(),
        "--completed".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_delete_all_force_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "42");
                assert_eq!(cmd.options.get("-a"), Some(&"true".to_string()));
                assert_eq!(cmd.options.get("-f"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(), "42".into(),
        "-a".into(),
        "-f".into()
    ];

    cli.parse(command);
}

#[test]
fn test_parse_delete_all_force_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "delete");
                assert_eq!(cmd.subcommand.1, "42");
                assert_eq!(cmd.options.get("--all"), Some(&"true".to_string()));
                assert_eq!(cmd.options.get("--force"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "delete".into(), "42".into(),
        "--all".into(),
        "--force".into()
    ];

    cli.parse(command);
}
