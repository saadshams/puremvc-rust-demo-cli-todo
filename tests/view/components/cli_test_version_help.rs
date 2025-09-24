use std::any::Any;
use std::sync::Arc;
use todo::model::value_object::Command;
use todo::view::components::CLI;

#[test]
fn test_parse_version_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("-v"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "-v".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_version_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--version"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "--version".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_help_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("-h"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "-h".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_help_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "");
                assert_eq!(cmd.subcommand.1, "");
                assert_eq!(cmd.options.get("--help"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "--help".into(),
    ];

    cli.parse(command);
}
