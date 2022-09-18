use convert_case::{Case, Casing};
use std::{env, path::PathBuf};

#[allow(deprecated)]
pub fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to get home directory");
        }
    }
}

pub fn get_ideas_dir() -> PathBuf {
    get_home_dir().join(".ideas")
}

pub fn get_idea_from_title(title: &str) -> PathBuf {
    get_ideas_dir().join(title.to_case(Case::Snake) + ".md")
}
