


use std::env;
use std::path::Path;

use crate::r#loop::main_loop;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, GetLastError};
use windows::Win32::System::Threading::{
    CreateEventW, OpenEventW, SetEvent, EVENT_MODIFY_STATE,
};

pub mod config;
pub mod global_variables;
pub mod keys;
pub mod r#loop;
#[path = "window/window.rs"]
pub mod window;

#[path = "scripts/window_features.rs"]
mod window_features;
#[path = "scripts/_try.rs"]
mod _try;

mod cli_args_config;

fn init() {
    let _ = global_variables::WORK_DIR.set(
        env::home_dir()
            .expect("Cannot get home directory")
            .join(".lixify")
            .join("better_hotkeys")
            .to_string_lossy()
            .to_string(),
    );

    let mut args = global_variables::ARGS.lock().unwrap();

    if args.config_path
        != global_variables::WORK_DIR
            .get()
            .expect("WORK_DIR is not set")
            .clone()
    {
        let config_path = Path::new(&args.config_path)
            .canonicalize()
            .expect("Cannot canonicalize config path");

        args.config_path = config_path.to_string_lossy().to_string();
    }


}

fn check_is_window_visible() {
    let args = global_variables::ARGS.lock().unwrap();

    if args.no_window {
        #[cfg(windows)]
        unsafe {
            windows::Win32::System::Console::FreeConsole().unwrap();
        }
    }
}

fn check_is_already_running() {
    let instance = &*global_variables::INSTANCE;

    let args = global_variables::ARGS.lock().unwrap();

    let event_name: Vec<u16> = "Global\\LixifyBetterHotkeysExit\0"
        .encode_utf16()
        .collect();

    if args.kill {
        unsafe {
            let event = OpenEventW(
                EVENT_MODIFY_STATE,
                false,
                PCWSTR(event_name.as_ptr()),
            );

            if let Ok(event) = event {
                SetEvent(event).expect("Cannot signal the running application to exit");
            } else if GetLastError() != ERROR_FILE_NOT_FOUND.into() {
                panic!("Cannot open the application exit event");
            }
        }
        std::process::exit(0);
    }

    if !instance.is_single() {
        std::process::exit(0);
    }

    let event = unsafe {
        CreateEventW(None, false, false, PCWSTR(event_name.as_ptr()))
            .expect("Cannot create the application exit event")
    };
    global_variables::EXIT_EVENT
        .set(event.0 as usize)
        .expect("The application exit event is already initialized");
}

pub fn check_exit_event() {
    let event = global_variables::EXIT_EVENT
        .get()
        .expect("EXIT_EVENT is not initialized");

    unsafe {
        if windows::Win32::System::Threading::WaitForSingleObject(
            windows::Win32::Foundation::HANDLE(*event as *mut std::ffi::c_void),
            0,
        )
            == windows::Win32::Foundation::WAIT_OBJECT_0
        {
            global_variables::LOOP.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

fn main() {
    init();
    check_is_window_visible();
    check_is_already_running();
    
    let args = global_variables::ARGS.lock().unwrap();
    let config_path = Path::new(&args.config_path);
    let config_path = if config_path.ends_with("config.toml") {
        config_path.to_path_buf()
    } else {
        config_path.join("config.toml")
    };

    let _ = global_variables::CONFIG
        .set(config::load(config_path.to_string_lossy().as_ref()).expect("Cannot load config"));

    global_variables::LOOP.store(true, std::sync::atomic::Ordering::Release);

    main_loop();
}
