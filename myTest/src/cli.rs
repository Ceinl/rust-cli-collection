use clap::{Arg, Command, ArgAction};

pub fn build_cli() -> Command {
    Command::new("ptd")
        .version("0.0.1")
        .author("Cein")
        .about("Cli todo-list")
        .arg(create_arg("add", 'a', "add", "Add new task to list"))
        .arg(create_arg("remove", 'r', "remove", "Remove task by 'id' from list"))
        .arg(create_arg("change", 'c', "change", "Change task status by 'id' from list"))
        .arg(create_arg("list", 'l', "list", "Display list of tasks"))
}

fn create_arg(name: &'static str, short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .short(short)
        .long(long)
        .help(help)
        .action(ArgAction::SetTrue) // Вказуємо, що цей аргумент є прапорцем
}
