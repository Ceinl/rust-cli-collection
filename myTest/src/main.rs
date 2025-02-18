mod cli;
mod cli_handlers;
mod cli_storage;
mod config;

fn main() 
{
    config::load_config();

    let matches = cli::build_cli().get_matches();
    cli_handlers::handle_matches(matches, &config::get_storage_path()); 
}
