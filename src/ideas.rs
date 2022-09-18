use crate::dir::get_idea_from_title;
use chrono::Utc;
use std::{
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
        panic!("File does not exist");
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
