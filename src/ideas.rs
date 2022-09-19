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
        stdin().read_line(&mut buf).unwrap();
        return match &buf[..] {
            "y\n" => {
                create_idea(dir.clone(), title, created_date);
            }
            _ => return println!("\nAborting..."),
        };
    } else {
        create_idea(dir.clone(), title, created_date);
    }
}

fn create_idea(dir: PathBuf, title: &str, created_date: i64) {
    let mut file = fs::File::create(dir.clone()).unwrap();
    writeln!(
        &mut file,
        "{}",
        format!(
            "---\ntitle: {}\ncreated-at: {}\n---\n\n<!-- Add your unique idea here! -->\n",
            title, created_date
        )
    )
    .unwrap();
    Command::new(var("EDITOR").unwrap_or(String::from("vim")))
        .arg(&dir)
        .arg("+call cursor(7, 0)")
        .status()
        .expect("Something went wrong...");
}

pub fn update_idea(title: &str) {
    let dir = get_idea_from_title(title);
    if !dir.exists() {
        println!("File does not exist");
        return;
    }

    Command::new(var("EDITOR").unwrap_or(String::from("vim")))
        .arg(&dir)
        .arg("+call cursor(7, 0)")
        .status()
        .expect("Something went wrong...");
}

pub fn delete_idea(title: &str) {
    let dir = get_idea_from_title(title);
    if !dir.exists() {
        return;
    }

    fs::remove_file(dir).unwrap();
}

pub fn list() {
    let dir = get_ideas_dir();
    let mut table = Table::new();
    table.set_header(vec!["Sl.", "Title", "Created At"]);
    let mut number = 1;

    for file in fs::read_dir(dir).unwrap() {
        let file = file.unwrap();
        let mut type_mark: HashMap<String, &str> = HashMap::new();
        type_mark.insert("created-at".into(), "i64");
        let content = fs::read_to_string(fs::DirEntry::path(&file)).expect("Unable to read file!");

        let meta = MetaData {
            content,
            required: vec!["title".to_string()],
            type_mark,
        };
        let meta = meta.parse().unwrap().0;
        let timestamp = meta
            .get("created-at")
            .unwrap()
            .clone()
            .as_string()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let date = datetime.format("%d-%m-%Y").to_string();
        table.add_row(vec![
            number.to_string(),
            meta.get("title").unwrap().clone().as_string().unwrap(),
            date,
        ]);
        number += 1;
    }
    println!("{}", table)
}

pub fn search(term: &str) {
    let dir = get_ideas_dir();
    let mut table = Table::new();
    table.set_header(vec!["Sl.", "Title", "Created At"]);
    let mut number = 1;
    for file in fs::read_dir(dir).unwrap() {
        let file = file.unwrap();
        let mut type_mark: HashMap<String, &str> = HashMap::new();
        type_mark.insert("created-at".into(), "i64");
        let content = fs::read_to_string(fs::DirEntry::path(&file)).expect("Unable to read file!");

        let meta = MetaData {
            content,
            required: vec!["title".to_string()],
            type_mark,
        };
        let meta = meta.parse().unwrap().0;
        let timestamp = meta
            .get("created-at")
            .unwrap()
            .clone()
            .as_string()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let date = datetime.format("%d-%m-%Y").to_string();
        let title = meta.get("title").unwrap().clone().as_string().unwrap();
        if title.to_lowercase().contains(term) {
            table.add_row(vec![
                number.to_string(),
                meta.get("title").unwrap().clone().as_string().unwrap(),
                date,
            ]);
            number += 1;
        }
    }
    println!("{}", table)
}
