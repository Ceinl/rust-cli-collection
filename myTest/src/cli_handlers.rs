use crate::cli_storage;
use std::io::{self, Write};

// Handlers
pub fn handle_matches(matches: clap::ArgMatches, _file_path: &str) {
    if matches.contains_id("add") {
        add_event(_file_path);
    } else if let Some(id) = matches.get_one::<String>("remove"){
        let id = id.parse::<u32>().expect("wrong argument");
        remove_event(_file_path, id);
    } else if matches.get_one::<String>("change").is_some() {
        change_status_event();
    } else if matches.contains_id("list") {
        list_event();
    } else {
        help_event();
    }
}


fn add_event(_file_path: &str) 
{
    let mut tasks = cli_storage::load_files(_file_path); 
    tasks.push(create_task(tasks.len() as u32 + 1));
    cli_storage::save_tasks(_file_path, &tasks);
}

fn create_task(id: u32) -> cli_storage::Task
{
    let name =  read_input("Enter task name: ");
    let content = read_input("Enter task desk: ");
    let status = false;
    cli_storage::Task { id, name, content, status }
}

fn remove_event(_file_path: &str, _id: u32) 
{

    

}

fn change_status_event() { }
fn list_event() { }
fn help_event() { }


fn read_input(prompt: &str) -> String
{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}





