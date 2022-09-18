mod cli;
mod dir;
mod notes;

use clap::Parser;
use cli::{Cli, Commands::*};
use dir::get_home_dir;
use notes::*;
use std::fs;

fn main() {
    let _ = init();
    let cli = Cli::parse();

    match &cli.command {
        Some(New { title }) => new_note(title),
        Some(Update { title }) => update_note(title),
        Some(Delete { title }) => delete_note(title),
        _ => {}
    };
}

fn init() -> Result<(), std::io::Error> {
    fs::create_dir_all(get_home_dir().join(".ideas")).unwrap();
    Ok(())
}
