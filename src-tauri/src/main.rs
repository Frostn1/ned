mod monitor;
mod border;
mod enum_monitors;

use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use crate::border::Border;
use crate::{monitor::Monitor, enum_monitors::enumerate_monitors};

#[tauri::command]
fn display_monitors() -> Vec<Monitor> {
    let mut monitors: Vec<Monitor> = vec![];
    for monitor in enumerate_monitors() {
        // Convert the WCHAR[] to a unicode OsString
        let name = match &monitor.szDevice[..].iter().position(|c| *c == 0) {
            Some(len) => OsString::from_wide(&monitor.szDevice[0..*len]),
            None => OsString::from_wide(&monitor.szDevice[0..monitor.szDevice.len()]),
        };
        let curr_border = Border {
            left: monitor.rcWork.left,
            top: monitor.rcWork.top,
            right: monitor.rcWork.right,
            bottom: monitor.rcWork.bottom,
        };
        let width = i32::abs(curr_border.left - curr_border.right);
        let height = i32::abs(curr_border.top - curr_border.bottom);
        monitors.push(Monitor {
            name: name.clone().into_string().unwrap(),
            borders: curr_border,
            width,
            height
        });
    }
    monitors
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![display_monitors])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
