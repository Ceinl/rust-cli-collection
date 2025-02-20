use terminal_size::{terminal_size, Height, Width };
use crate::cli_storage::{self, Task};


struct TableConfig 
    {
        screen_width: u16,
        max_table_width: u16,
        min_table_width:u16,
        min_col_count:u16,
        max_col_count:u16,
        margin:u16,
    }

struct TableResult 
    {
    width: u16,
    col_count: u16,
    margin: u16,
    }

pub fn list_event(_file_path: &str) 
{
    let tasks = cli_storage::load_files(_file_path); 
    let (Width(w), _) = terminal_size().unwrap();

    let table_config = TableConfig{
    screen_width: w,
    max_table_width: 100,
    min_table_width: 25,
    min_col_count: 1,
    max_col_count: 5,
    margin: 2,
    };

    print_tables(tasks,table_config);
}

fn print_tables(tasks: Vec<cli_storage::Task>, config: TableConfig) {
    let data = get_table_width(config);
    let tables: Vec<Vec<String>> = tasks
        .into_iter()
        .map(|task| get_table(data.width, task))
        .collect();
    let margin_str = " ".repeat(data.margin as usize);

    for chunk in tables.chunks(data.col_count as usize) {
        let max_height = chunk.iter().map(|t| t.len()).max().unwrap_or(0);
        for row in 0..max_height {
            let line_parts: Vec<String> = chunk
                .iter()
                .map(|table| {
                    if row < table.len() {
                        table[row].clone()
                    } else {
                        " ".repeat(data.width as usize)
                    }
                })
                .collect();
            println!("{}{}", margin_str, line_parts.join(&margin_str));
        }
        println!();
    }
}

fn get_table(width: u16, task: cli_storage::Task) -> Vec<String>
{
    let mut table: Vec<String> = Vec::new();
    let close_line = get_close_line(width);
    let status_line = get_status_line(task.id, task.name, task.status, width);
    let content = get_content_line(width, task.content);
    table.push(close_line.clone());
    table.push(status_line);
    table.push(close_line.clone());
    table.extend(content);
    table.push(close_line);
    table
}


fn get_table_width(config: TableConfig) -> TableResult
{
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
    
    TableResult{width: optimal_width, col_count: col_count, margin: config.margin}
    
}

fn get_close_line(len:u16) -> String 
{
    format!("+{}+", "-".repeat((len - 2) as usize))
} 

fn get_status_line(id: u32, name: String, status: bool, width: u16) -> String {
    let id_field = format!("| {} |", id);
    let status_field = format!("{} |", status);
    let width = width as usize;
    let name_field_width = width - id_field.len() - status_field.len();
    let truncated_name: String = name.chars().take(name_field_width).collect();
    let padded_name = format!(
        "{}{}",
        truncated_name,
        " ".repeat(name_field_width.saturating_sub(truncated_name.len()))
    );
    format!("{}{}{}", id_field, padded_name, status_field)
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









