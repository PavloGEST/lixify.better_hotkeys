use crate::{global_variables, window_features};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

pub fn main_loop() {
    let config = global_variables::CONFIG.get().expect("CONFIG is not set");

    window_features::begin(config.get_active_window_features());

    while global_variables::LOOP.load(Ordering::Acquire) {
        crate::check_exit_event();
        thread::sleep(Duration::from_millis(10));
    }
}
