mod border;
mod db;
mod enum_monitors;
mod monitor;

use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use db::{read_from_db, write_new_monitors};
use serde_json::Value;

use crate::border::Border;
use crate::{enum_monitors::enumerate_monitors, monitor::Monitor};

#[tauri::command]
fn search_monitors() {
    let mut monitors: Vec<Monitor> = vec![];
    for monitor in enumerate_monitors() {
        let name = match &monitor.szDevice[..].iter().position(|c| *c == 0) {
            Some(len) => OsString::from_wide(&monitor.szDevice[0..*len]),
            None => OsString::from_wide(&monitor.szDevice[0..monitor.szDevice.len()]),
        };
        let curr_border = Border {
            left: monitor.rcMonitor.left,
            top: monitor.rcMonitor.top,
            right: monitor.rcMonitor.right,
            bottom: monitor.rcMonitor.bottom,
        };
        let width = i32::abs(curr_border.left - curr_border.right);
        let height = i32::abs(curr_border.top - curr_border.bottom);
        monitors.push(Monitor {
            name: name.clone().into_string().unwrap(),
            borders: curr_border,
            width,
            height,
        });
    }
    write_new_monitors(monitors);
    // return monitors;
}

#[tauri::command]
fn fetch_monitors() -> Value {
    // search_monitors();
    let db = read_from_db();
    db["monitors"].to_owned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::read_from_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_monitors, fetch_monitors])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
