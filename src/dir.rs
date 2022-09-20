use convert_case::{Case, Casing};
use home::home_dir;
use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    home_dir().expect("Unable to get home directory.")
}

pub fn get_ideas_dir() -> PathBuf {
    get_home_dir().join(".ideas")
}

pub fn get_idea_from_title(title: &str) -> PathBuf {
    get_ideas_dir().join(title.to_case(Case::Snake) + ".md")
}
