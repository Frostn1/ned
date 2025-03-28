use std::io::Error;
use std::mem;
use std::ptr;

use winapi::shared::minwindef::{LPARAM, TRUE, BOOL};
use winapi::shared::windef::{HMONITOR, HDC, LPRECT};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};


unsafe extern "system" fn enumerate_monitors_callback(
    monitor: HMONITOR,
    _: HDC,
    _: LPRECT,
    userdata: LPARAM,
) -> BOOL {
    // Get the userdata where we will store the result
    let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);

    // Initialize the MONITORINFOEXW structure and get a pointer to it
    let mut monitor_info: MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    // Call the GetMonitorInfoW win32 API
    let result = GetMonitorInfoW(monitor, monitor_info_ptr);
    if result == TRUE {
        // Push the information we received to userdata
        monitors.push(monitor_info);
    }

    TRUE
}

pub(crate) fn enumerate_monitors() -> Vec<MONITORINFOEXW> {
    // Define the vector where we will store the result
    let mut monitors = Vec::<MONITORINFOEXW>::new();
    let userdata = &mut monitors as *mut _;

    let result = unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(enumerate_monitors_callback),
            userdata as LPARAM,
        )
    };

    if result != TRUE {
        // Get the last error for the current thread.
        // This is analogous to calling the Win32 API GetLastError.
        panic!("Could not enumerate monitors: {}", Error::last_os_error());
    }

    monitors
}
