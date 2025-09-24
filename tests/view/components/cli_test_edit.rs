use std::any::Any;
use std::sync::Arc;
use todo::model::value_object::Command;
use todo::view::components::CLI;

#[test]
fn test_parse_edit() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "edit");
                assert_eq!(cmd.subcommand.1, "21");
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "edit".into(), "21".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_edit_title_short() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "edit");
                assert_eq!(cmd.subcommand.1, "21");
                assert_eq!(cmd.options.get("-t"), Some(&"Water the plants".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "edit".into(), "21".into(),
        "-t".into(), "Water the plants".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_edit_title_long() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "edit");
                assert_eq!(cmd.subcommand.1, "21");
                assert_eq!(cmd.options.get("--title"), Some(&"Water the plants".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "edit".into(), "21".into(),
        "--title".into(), "Water the plants".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_edit_title_active() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "edit");
                assert_eq!(cmd.subcommand.1, "21");
                assert_eq!(cmd.options.get("-t"), Some(&"Water the plants".to_string()));
                assert_eq!(cmd.options.get("--active"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "edit".into(), "21".into(),
        "-t".into(), "Water the plants".into(),
        "--active".into(),
    ];

    cli.parse(command);
}

#[test]
fn test_parse_edit_completed() {
    let notify: Arc<dyn Fn(Arc<dyn Any + Send + Sync>) + Send + Sync> = {
        Arc::new(move |command: Arc<dyn Any + Send + Sync>| {
            if let Some(cmd) = command.downcast_ref::<Command>() {
                assert_eq!(cmd.subcommand.0, "edit");
                assert_eq!(cmd.subcommand.1, "21");
                assert_eq!(cmd.options.get("--completed"), Some(&"true".to_string()));
            }
        })
    };

    let mut cli = CLI::new();
    cli.set_delegate(notify);

    let command = vec![
        "target/debug/todo".into(),
        "edit".into(), "21".into(),
        "--completed".into(),
    ];

    cli.parse(command);
}
