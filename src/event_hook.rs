use std::ffi::c_ulong;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::prelude::OsStringExt;
use core::ffi::c_void;
use core::ffi::c_int;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::Graphics::Dwm::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::System::SystemServices::IMAGE_DOS_HEADER,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Accessibility::*,
};

use crate::border::WindowBorder;
use crate::border::BORDER_POINTER;
use crate::border::FACTORY_POINTER;

pub extern "system" fn handle_win_event(
    h_win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    id_object: i32,
    id_child: i32,
    dw_event_thread: u32,
    dwms_event_time: u32,
) {
    if id_object == OBJID_CURSOR.0 {
        return;
    }
    match event {
        EVENT_OBJECT_LOCATIONCHANGE => {
            let border_pointer = BORDER_POINTER.get().unwrap();
            let factory_pointer = FACTORY_POINTER.get().unwrap();
            //Pretty unsafe code but ehhh it's probably fine I'm a C programmer at heart anyways
            //(not that I was ever a good one).
            unsafe { (*border_pointer).update(&*factory_pointer) };
        },
        EVENT_SYSTEM_FOREGROUND => {
            println!("focus? {:?}", hwnd);
            let border_pointer = BORDER_POINTER.get().unwrap();
            let factory_pointer = FACTORY_POINTER.get().unwrap();
            unsafe { (*border_pointer).set_pos() };

            // TODO Code below doesn't work. I think I can just move this into the border structure
            // itself (specifically in the update function) and maybe add a bool to the arguments
            // of update to signify whether I want to reset border color/position or not.
            /*let focused_window = unsafe { GetForegroundWindow() };
            println!("focused_window: {:?}", focused_window);
            match unsafe{ (*border_pointer).m_tracking_window } {
                focused_window => {
                    let r: f32 = 152.0/255.0;
                    let g: f32 = 152.0/255.0;
                    let b: f32 = 152.0/255.0;
                    unsafe { (*border_pointer).set_color(r, g, b, &(*factory_pointer)) };
                },
                _ => {
                    let r: f32 = 80.0/255.0;
                    let g: f32 = 80.0/255.0;
                    let b: f32 = 80.0/255.0;
                    unsafe { (*border_pointer).set_color(r, g, b, &(*factory_pointer)) };
                }
            }*/
        },
        EVENT_OBJECT_HIDE => {
            let border_pointer = BORDER_POINTER.get().unwrap();
            unsafe { ShowWindow((*border_pointer).m_window, SW_HIDE) };
        },
        EVENT_OBJECT_SHOW => {
            let border_pointer = BORDER_POINTER.get().unwrap();
            unsafe { ShowWindow((*border_pointer).m_window, SW_SHOWNA) };
        },
        EVENT_OBJECT_DESTROY => {
            let mut border_pointer = BORDER_POINTER.get().unwrap();
            let hwnd = unsafe{ (*border_pointer).m_window };
            println!("Destroying border window! {:?}", hwnd);
            unsafe { DestroyWindow(hwnd) };
        },
        _ => {}
    }
    //println!("HWINEVENTHOOK: {:?}", h_win_event_hook);
    //std::thread::sleep(std::time::Duration::from_millis(100));
}

pub extern "system" fn handle_win_event_main(
    h_win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    id_object: i32,
    id_child: i32,
    dw_event_thread: u32,
    dwms_event_time: u32,
) {
    match event {
        EVENT_OBJECT_CREATE => {
            if unsafe { IsWindowVisible(hwnd).as_bool() } {
                unsafe { UnhookWinEvent(h_win_event_hook) };

                println!("window created! {:?}", hwnd);

                unsafe {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    SetWinEventHook(
                        EVENT_MIN,
                        EVENT_MAX,
                        None,
                        Some(handle_win_event_main),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
                    );
                }
            }
            /*unsafe {
                if IsWindowVisible(hwnd).as_bool() {
                    //println!("In enum_windows_callback and window is visible!");
                    let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;

                    // Exclude certain window styles like WS_EX_TOOLWINDOW
                    if ex_style & WS_EX_TOOLWINDOW.0 == 0 && style & WS_POPUP.0 == 0 {
                        println!("valid window!");
                    }
                }
            }*/
        },
        _ => {}
    }
}