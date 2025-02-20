use terminal_size::{terminal_size,Width, };
use crate::cli_storage;


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
    screen_width: 500,
    max_table_width: 100,
    min_table_width: 25,
    min_col_count: 1,
    max_col_count: 5,
    margin: 2,
    };
    print_table(tasks,table_config);
}

fn print_table(tasks: Vec<cli_storage::Task>, config: TableConfig) {
    let data = get_table_width(config);
    let content_height = get_table_height("123123".to_string(), data.0, data.2); 
    let mut tables: Vec<String> = vec![String::new(); content_height.into()];
    let close_line = get_close_line(data.0);
    let status_line = get_status_line(25, "Example".to_string(), true, data.0);
    let double_close = format!("{} {}", close_line, close_line);
    let double_status = format!("{} {}", status_line, status_line);
    tables.push(double_close.clone());
    tables.push(double_status);
    tables.push(double_close.clone());
    tables.extend(get_content_line(data.0, "asdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasd".to_string()));
    tables.push(double_close);
    for line in tables {
        println!("{}", line);
    }
}
fn get_table_width(config: TableConfig) -> (u16, u16, u16) {
    //if let Some((Width(w), Height(_))) = terminal_size() {
    //   println!("{}", w);
    //}
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

fn get_close_line(len:u16) -> String 
{
    format!("+{}+", "-".repeat((len - 2) as usize))
} 

fn get_status_line(id: u16, name: String, status: bool, width: u16) -> String {
    let _id = format!("| {} |", id);
    let _status = format!("{} |", status);
    let name_max_len = width as usize - _id.len() - _status.len();
    let _name = format!("{}{}|", name, " ".repeat(name_max_len.saturating_sub(1)));
    format!("{}{}{}", _id, _name, _status)
}

fn get_content_line(width: u16, content: String) -> Vec<String> 
{
    let max_len = (width as usize).saturating_sub(4);
    let mut lines: Vec<String> = Vec::new();
    let mut start = 0;
    while start < content.len() 
    {
        let end = (start + max_len).min(content.len());
        let part = &content[start..end];
        lines.push(format!("| {}{} |",part, " ".repeat(max_len - part.len())));
        start = end;
    }
   lines 
}









