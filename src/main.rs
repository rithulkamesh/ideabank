mod cli;
mod dir;
mod ideas;

use clap::Parser;
use cli::{Cli, Commands::*};
use dir::get_home_dir;
use ideas::*;
use std::fs;

fn main() {
    let _ = init();
    let cli = Cli::parse();

    match &cli.command {
        Some(New { title }) => new_idea(&title),
        Some(Update { title }) => update_idea(&title),
        Some(Delete { title }) => delete_idea(&title),
        Some(List {}) => list(),
        Some(Search { term }) => search(&term),
        _ => {
            println!("\nInvalid Command, Run `ideabank help` for help with using the CLI")
        }
    };
}

fn init() -> Result<(), std::io::Error> {
    fs::create_dir_all(get_home_dir().join(".ideas")).unwrap();
    Ok(())
}
