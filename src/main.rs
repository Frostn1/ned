use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};
use std::process::Command;


fn get_mouse_pos() -> POINT {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point as *mut POINT) };
    return point;
}

fn set_mouse_pos(pos: POINT) {
    unsafe {
        SetCursorPos(pos.x, pos.y);
    }
    return;
}

fn main() {
    // store mouse postion
    loop {
        let pos = get_mouse_pos();

        // print mouse coords
        println!("pos is {:?}", pos);
        let mut child = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = child.wait().unwrap();
    }

    

    // mouse mouse
    // set_mouse_pos(pos);

}
