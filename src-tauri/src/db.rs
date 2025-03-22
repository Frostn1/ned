use std::path::{Path, PathBuf};
use std::{fs, path};

use crate::monitor::Monitor;

// const DB_FILE_PATH: String =;

fn try_open_db(file_path: String) {
    if Path::new(&file_path).exists() {
        // let srcdir = PathBuf::from(&file_path);
        // println!("{:?}", fs::canonicalize(&srcdir));
        return;
    }
    println!("creating file");
    fs::write(file_path, b"{\"displays\":[]}").expect("Unable to write file");
}

pub(crate) fn read_from_db() -> serde_json::Value {
    try_open_db("displays.json".to_owned());

    let content = fs::read_to_string("displays.json").expect("file couldn't open");
    let json: serde_json::Value =
        serde_json::from_str(if !content.is_empty() { &content } else { "{\"displays\":[]}" })
            .expect("JSON was not well-formatted");
    json
}

pub(crate) fn write_new_monitors(new_monitors: Vec<Monitor>) {
    let db = read_from_db();
    // let mut final_monitors: Vec<Monitor> = vec![];

    println!("known monitors");
    if let serde_json::Value::Array(knowns) = &db["displays"] {
        for known_monitor in knowns {
            println!("{:?}", known_monitor);
        }
    }
    println!("new monitors");
    for new_monitor in new_monitors {
        println!("{:?}", new_monitor);
    }
}
