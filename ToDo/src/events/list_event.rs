use std::cmp::min_by;

use terminal_size::{terminal_size,Width, Height};
use crate::cli_storage;

// Acrual_colum = screen_width / ((table_width + gap) * column )

struct TableConfig 
    {
        screen_width: u16,
        max_table_width: u16,
        min_table_width:u16,
        min_col_count:u16,
        max_col_count:u16,
        margin:u16,
    }

pub fn list_event(_file_path: &str) 
{
   let tasks = cli_storage::load_files(_file_path); 

    let table_config = TableConfig{
    screen_width: 252,
    max_table_width: 50,
    min_table_width: 25,
    min_col_count: 1,
    max_col_count: 5,
    margin: 2,
    };
    print_table(tasks,table_config);
}

fn print_table(tasks: Vec<cli_storage::Task>, config: TableConfig) 
{

    let data = get_table_width(config);
    let content_height = get_table_height("123123".to_string(), data.0, data.2); 

    // NONONO
    let mut table: Vec<String> = Vec::new(); 
    table.push(get_close_line(data.0,data.2));
    
    table.push(get_close_line(data.0,data.2));
}


fn get_table_width(config: TableConfig) -> (u16, u16, u16) {
    if let Some((Width(w), Height(_))) = terminal_size() {
        println!("{}", w);
    }
    let possible_cols = ((config.screen_width - config.margin) as f32
        / (config.min_table_width + config.margin) as f32)
        .round() as u16;
    let col_count = config.max_col_count.min(possible_cols);
    if col_count < config.min_col_count {
        panic!("Cant reach minimal column number, please check a config");
    }
    let optimal_width = (config.screen_width - (col_count + 1) * config.margin) / col_count;
    let optimal_width = config.max_table_width.min(optimal_width);
    if optimal_width < config.min_table_width {
        panic!("Cant reach minimal table width, please check a config");
    }
    (optimal_width, col_count, config.margin)
}

fn get_table_height(content: String, width: u16, margin: u16) -> u16 {
    let content_width = width.saturating_sub(margin * 2 + 2).max(1) as usize;
    let content_height = (content.len() as f64 / content_width as f64).ceil() as u16;
    content_height
}

fn get_close_line(len:u16, margin: u16) -> String 
{
    format!("+{}+", "*".repeat((len - 2) as usize))
} 














