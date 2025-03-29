
use std::process::Command;
use crate::db;
use serde_json::from_value;

use winapi::{shared::windef::POINT, um::winuser::GetCursorPos};

use crate::Monitor;

fn get_mouse_pos() -> POINT {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) };
    point
}


fn find_monitor_under_cursor(pos: POINT, monitors: Vec<Monitor>) -> Option<Monitor> {
    for monitor in monitors {
        if pos.x >= monitor.border.left
            && pos.x <= monitor.border.right
            && pos.y >= monitor.border.top
            && pos.y <= monitor.border.bottom
        {
            return Some(monitor.clone());
        }
    }
    None
}

pub(crate) fn check_cursor() {
    let db = db::read_from_db();
    let monitors = db["monitors"].to_owned();
    println!("Monitors: {:?}", monitors);

    let pos = get_mouse_pos();
    let monitors: Vec<Monitor> = match db["monitors"].as_array() {
        Some(known_monitors) => known_monitors
            .iter()
            .filter_map(|m| from_value(m.clone()).ok()) // Deserialize each monitor
            .collect(),
        None => vec![], // Handle the case where "monitors" is not an array
    };
    let monitor = find_monitor_under_cursor(pos, monitors);


    match monitor {
        Some(monitor) => {
            println!("Cursor is in monitor: {:?}", monitor);
        }
        None => {
            println!("Cursor is outside all monitors! x: {} y: {}", pos.x, pos.y);
        }
    }

    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    let _result = child.wait().unwrap();
}
