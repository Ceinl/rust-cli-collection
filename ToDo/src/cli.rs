use clap::{Arg, Command, ArgAction};

fn create_flag(name: &'static str, short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .short(short)
        .long(long)
        .help(help)
        .action(ArgAction::SetTrue)
}

fn create_arg_with_value(name: &'static str, short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .short(short)
        .long(long)
        .help(help)
        .required(false)
        .num_args(1) 
}

pub fn build_cli() -> Command {
    Command::new("ptd")
        .version("0.0.1")
        .author("Cein")
        .about("Cli todo-list")
        .arg(create_flag("add", 'a', "add", "Add new task to list"))
        .arg(create_arg_with_value("remove", 'r', "remove", "Remove task by 'id' from list"))
        .arg(create_arg_with_value("change", 'c', "change", "Change task status by 'id' from list"))
        .arg(create_flag("list", 'l', "list", "Display list of tasks"))
}
