//
// module to load and save tasks 
//

use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task 
    {
        pub id: u32,
        pub name: String,
        pub content: String,
        pub status: bool,
    }


fn _save_tasks(file_path: &str, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> 
{
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn save_tasks(file_path: &str, tasks: &[Task]) {
    match _save_tasks(file_path, tasks) {
        Ok(()) => (), 
        Err(e) => eprintln!("Error saving tasks: {}", e), 
    }
}


fn _load_files(file_path: &str ) -> Result<Vec<Task>,Box<dyn std::error::Error>> 
{
    let data = fs::read_to_string(file_path)?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn load_files(file_path: &str ) -> Vec<Task> 
{
  match _load_files(file_path) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            Vec::new()
        }
    }} 


