use terminal_size::{terminal_size,Width};
use crate::cli_storage;
use std::io::{self, Write};
 
// Handlers
pub fn handle_matches(matches: clap::ArgMatches, _file_path: &str) {
    if matches.get_flag("add") { 
        add_event(_file_path);
    } else if let Some(id) = matches.get_one::<String>("remove") {
        let id = id.parse::<u32>().expect("wrong argument");
        remove_event(_file_path, id);
    } else if let Some(id) =  matches.get_one::<String>("change") {
        let id = id.parse::<u32>().expect("wrong argument");
        change_status_event(_file_path,id);
    } else if matches.get_flag("list") {
        list_event(_file_path);
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
    let mut tasks = cli_storage::load_files(_file_path); 
    tasks.retain(|task| task.id != _id);
    for (index, task) in tasks.iter_mut().enumerate()
    {
        task.id = (index + 1) as u32;
    }
    cli_storage::save_tasks(_file_path, &tasks);
    }


fn change_status_event(_file_path: &str, id: u32 ) 
{
    let mut tasks = cli_storage::load_files(_file_path); 
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) 
    {
        task.status = !task.status;
    }
    cli_storage::save_tasks(_file_path, &tasks);
}
fn list_event(_file_path: &str) 
{
    let tasks = cli_storage::load_files(_file_path); 
    print_table("");
    
}
fn help_event() { }


fn read_input(prompt: &str) -> String
{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_table(prompt: &str) 
{
   if let Some((Width(w), _)) = terminal_size() 
    {
        println!("{}",w);
    }
}



