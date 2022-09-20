use crate::dir::{get_idea_from_title, get_ideas_dir};
use chrono::{DateTime, NaiveDateTime, Utc};
use comfy_table::Table;
use markdown_meta_parser::MetaData;
use std::{
    collections::HashMap,
    env::var,
    fs,
    io::{stdin, Write},
    path::PathBuf,
    process::Command,
};
pub fn new_idea(title: &str) {
    let dir = get_idea_from_title(title);
    let created_date = Utc::now().timestamp();
    if dir.exists() {
        println!("There already exists a idea with this name. Proceed? (y/n)");
        let mut buf = String::from("");
        stdin()
            .read_line(&mut buf)
            .expect("Unable to read from STDIN");
        return match &buf[..] {
            "y\n" => create_idea(dir, title, created_date),
            _ => return println!("\nAborting..."),
        };
    } else {
        create_idea(dir, title, created_date);
    }
}

fn create_idea(dir: PathBuf, title: &str, created_date: i64) {
    let mut file = fs::File::create(&dir).expect("Permission denied to create file");
    writeln!(
        &mut file,
        "{}",
        format!(
            "---\ntitle: {}\ncreated-at: {}\n---\n\n<!-- Add your unique idea here! -->\n",
            title, created_date
        )
    )
    .expect("Permission Denied");
    open_editor(dir);
}

fn open_editor(dir: PathBuf) {
    Command::new(var("EDITOR").unwrap_or(String::from("vim")))
        .arg(&dir)
        .arg("+call cursor(7, 0)")
        .status()
        .expect("Failed to open editor");
}
pub fn update_idea(title: &str) {
    let dir = get_idea_from_title(title);
    if !dir.exists() {
        println!("File does not exist");
        return;
    }

    open_editor(dir);
}

pub fn delete_idea(title: &str) {
    let dir = get_idea_from_title(title);
    if !dir.exists() {
        return;
    }
    fs::remove_file(dir).expect("Failed to remove file");
}

pub fn list() {
    get_ideas(|_title| true);
}

pub fn search(term: &str) {
    get_ideas(|title| title.to_lowercase().contains(&term.to_lowercase()));
}

fn get_ideas<F: Fn(&str) -> bool>(filter: F) {
    let dir = get_ideas_dir();
    let mut table = Table::new();
    table.set_header(vec!["Sl.", "Title", "Created At"]);
    let mut number = 1;

    for file in fs::read_dir(dir).expect("Unable to read ideas directory.") {
        let file = file.expect("Failed to read file");
        let mut type_mark: HashMap<String, &str> = HashMap::new();
        type_mark.insert("created-at".into(), "i64");
        let content = fs::read_to_string(fs::DirEntry::path(&file)).expect("Unable to read file!");

        let meta = MetaData {
            content,
            required: vec!["title".to_string()],
            type_mark,
        };
        let meta = meta.parse().expect("Failed to get meta").0;
        let timestamp = meta
            .get("created-at")
            .expect("Failed to get meta")
            .clone()
            .as_string()
            .expect("Failed to get meta")
            .parse::<i64>()
            .expect("Failed to get meta");
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let date = datetime.format("%d-%m-%Y").to_string();
        let title = meta
            .get("title")
            .expect("Failed to get meta")
            .clone()
            .as_string()
            .expect("Failed to get meta");
        if filter(&title) {
            table.add_row(vec![number.to_string(), title, date]);
            number += 1;
        }
    }
    println!("{}", table)
}
