use std::path::Path;
use std::fs;

use crate::monitor::Monitor;
use std::collections::HashMap;

static DB_FILE_PATH: &'static str = "db.json";


fn try_open_db(file_path: String) {
    if Path::new(&file_path).exists() {
        // let srcdir = PathBuf::from(&file_path);
        // println!("{:?}", fs::canonicalize(&srcdir));
        return;
    }
    println!("creating file");
    fs::write(file_path, b"{\"monitors\":[]}").expect("Unable to write file");
}

pub(crate) fn read_from_db() -> serde_json::Value {
    try_open_db(DB_FILE_PATH.to_owned());

    let content = fs::read_to_string(DB_FILE_PATH.to_owned()).expect("file couldn't open");
    let json: serde_json::Value =
        serde_json::from_str(if !content.is_empty() { &content } else { "{\"monitors\":[]}" })
            .expect("JSON was not well-formatted");
    json
}

pub(crate) fn write_to_db(value: serde_json::Value) {
    try_open_db(DB_FILE_PATH.to_owned());
    fs::write(DB_FILE_PATH.to_owned(), value.to_string()).expect("Unable to write file");
}

pub(crate) fn write_new_monitors(new_monitors: Vec<Monitor>) {
    let mut db = read_from_db();

    // Extract existing monitors, ensuring it's an array
    let known_monitors: Vec<Monitor> = match db.get("monitors") {
        Some(serde_json::Value::Array(existing)) => existing
            .iter()
            .filter_map(|m| serde_json::from_value(m.clone()).ok())
            .collect(),
        _ => vec![],
    };

    // Create a HashMap to replace monitors by name
    let mut monitor_map: HashMap<String, Monitor> = known_monitors
        .into_iter()
        .map(|monitor| (monitor.name.clone(), monitor))
        .collect();

    // Insert or replace monitors with the same name
    for new_monitor in new_monitors {
        monitor_map.insert(new_monitor.name.clone(), new_monitor);
    }

    // Convert updated monitors back to JSON
    let updated_monitors: Vec<serde_json::Value> = monitor_map
        .into_values()
        .map(|monitor| serde_json::to_value(monitor).expect("Failed to serialize monitor"))
        .collect();

    // Update and write back to the database
    db["monitors"] = serde_json::Value::Array(updated_monitors);
    write_to_db(db);
}
