use crate::_try::*;
use crate::{
    config::{ActiveWindowFeature, FeatureAction},
    global_variables,
    keys::{Hotkey, build_hotkeys},
    window::{Window, WindowDrag},
};
use rdev::{Button, Event, EventType, Key, grab, simulate};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn set_window_fullscreen() {
    if let Err(err) = _try_set_window_fullscreen() {
        eprintln!("Window error: {}", err);
    }
}

pub fn quick_close_window() {
    _try_quick_close_window().unwrap_or_else(|err| eprintln!("Window error: {}", err));
}

pub fn move_window_to_desktop(number: u32) {
    let config = global_variables::CONFIG.get().expect("CONFIG is not set");
    let result = if config
        .window_features
        .move_window_to_desktop
        .additional_flags
        .as_ref()
        .map_or(false, |flags| flags.iter().any(|flag| flag == "auto_slide"))
    {
        _try_move_and_slide_to_desktop(number)
    } else {
        _try_move_window_to_desktop(number)
    };
    result.unwrap_or_else(|err| eprintln!("Window error: {}", err));
}

pub fn slide_to_desktop(number: u32) {
    _try_slide_to_desktop(number).unwrap_or_else(|err| eprintln!("Window error: {}", err));
}

pub fn begin(features: Vec<ActiveWindowFeature>) {
    let drag_feature = features
        .iter()
        .find(|feature| matches!(feature.function, FeatureAction::Drag));
    let drag_modifiers: HashSet<Key> = drag_feature
        .map(|feature| feature.config.key_bindings.iter().copied().collect())
        .unwrap_or_default();
    // "mouse_wheel_only": drag with the middle button alone, no modifier keys.
    let wheel_only = drag_feature
        .and_then(|feature| feature.config.additional_flags.as_ref())
        .is_some_and(|flags| flags.iter().any(|flag| flag == "mouse_wheel_only"));
    let drag_button = if wheel_only {
        Button::Middle
    } else {
        Button::Left
    };
    let drag = RefCell::new(None::<DragHandle>);
    let middle_drag_window = RefCell::new(None::<PendingMiddleClick>);
    let bindings = RefCell::new(
        features
            .iter()
            .flat_map(build_hotkeys)
            .collect::<Vec<Hotkey>>(),
    );
    let pressed = RefCell::new(HashSet::<Key>::new());
    let blocked = RefCell::new(HashSet::<Key>::new());

    thread::spawn(move || {
        let result = grab(move |event: Event| {
            match event.event_type {
                EventType::ButtonPress(button) if button == Button::Middle && wheel_only => {
                    match winsafe::GetCursorPos() {
                        Ok(cursor) => match Window::under_cursor() {
                            Ok(window) => {
                                *middle_drag_window.borrow_mut() = Some(PendingMiddleClick {
                                    window,
                                    cursor_x: cursor.x,
                                    cursor_y: cursor.y,
                                });
                            }
                            Err(error) => eprintln!("Window selection error: {error}"),
                        },
                        Err(error) => eprintln!("Cursor position error: {error}"),
                    }
                    return Some(event);
                }

                EventType::ButtonPress(button)
                    if button == drag_button
                        && !wheel_only
                        && (!drag_modifiers.is_empty()
                            && drag_modifiers.is_subset(&pressed.borrow())) =>
                {
                    match Window::under_cursor().and_then(|window| window.start_drag()) {
                        Ok(Some(started)) => {
                            *drag.borrow_mut() = Some(spawn_drag(started));
                            suppress_start_menu();
                            return None;
                        }
                        Ok(None) => {}
                        Err(error) => eprintln!("Window drag error: {error}"),
                    }
                }

                EventType::MouseMove { x, y } if wheel_only => {
                    let should_start_drag =
                        middle_drag_window.borrow().as_ref().is_some_and(|pending| {
                            (x - f64::from(pending.cursor_x)).abs() >= DRAG_THRESHOLD
                                || (y - f64::from(pending.cursor_y)).abs() >= DRAG_THRESHOLD
                        });

                    if should_start_drag {
                        let pending = middle_drag_window.borrow_mut().take();
                        if let Some(pending) = pending {
                            match pending.window.start_drag() {
                                Ok(Some(started)) => {
                                    *drag.borrow_mut() = Some(spawn_drag(started));
                                }
                                Ok(None) => {}
                                Err(error) => eprintln!("Window drag error: {error}"),
                            }
                            return None;
                        }
                    }
                }

                EventType::ButtonRelease(Button::Middle) if wheel_only => {
                    if drag.borrow().is_some() {
                        *drag.borrow_mut() = None;
                    }
                    middle_drag_window.borrow_mut().take();
                    return Some(event);
                }

                EventType::ButtonRelease(button)
                    if button == drag_button && drag.borrow().is_some() =>
                {
                    *drag.borrow_mut() = None;
                    return None;
                }

                EventType::ButtonPress(_) | EventType::ButtonRelease(_) => {
                    if let Err(error) = _try_select_window_under_cursor() {
                        eprintln!("Window selection error: {error}");
                    }
                }

                EventType::KeyPress(key) => {
                    let mut pressed = pressed.borrow_mut();
                    if !pressed.insert(key) {
                        return if blocked.borrow().contains(&key) {
                            None
                        } else {
                            Some(event)
                        };
                    }

                    let mut swallow = false;
                    for hotkey in bindings.borrow_mut().iter_mut() {
                        if key == hotkey.trigger
                            && !hotkey.triggered
                            && if hotkey.exact {
                                hotkey.keys == *pressed
                            } else {
                                hotkey.keys.is_subset(&pressed)
                            }
                        {
                            hotkey.triggered = true;

                            #[cfg(debug_assertions)]
                            println!("hotkey triggered: {}", hotkey.name);

                            (hotkey.callback)();

                            swallow |= hotkey.exact;
                        }
                    }

                    if swallow {
                        blocked.borrow_mut().insert(key);
                        suppress_start_menu();
                        return None;
                    }
                }

                EventType::KeyRelease(key) => {
                    pressed.borrow_mut().remove(&key);

                    for hotkey in bindings.borrow_mut().iter_mut() {
                        if key == hotkey.trigger {
                            hotkey.triggered = false;
                        }
                    }

                    if blocked.borrow_mut().remove(&key) {
                        return None;
                    }
                }

                _ => {}
            }

            Some(event)
        });

        if let Err(error) = result {
            eprintln!("failed to grab global keyboard events: {error:?}");

            global_variables::LOOP.store(false, Ordering::Release);
        }
    });
}

const DRAG_THRESHOLD: f64 = 5.0;

struct PendingMiddleClick {
    window: Window,
    cursor_x: i32,
    cursor_y: i32,
}

struct DragHandle(Arc<AtomicBool>);

impl Drop for DragHandle {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

fn spawn_drag(drag: WindowDrag) -> DragHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);

    thread::spawn(move || {
        while !thread_stop.load(Ordering::Acquire) {
            if let Err(error) = drag.update() {
                eprintln!("Window drag error: {error}");
                break;
            }
            thread::sleep(Duration::from_millis(4));
        }
    });

    DragHandle(stop)
}

fn suppress_start_menu() {
    thread::spawn(|| {
        let dummy = Key::Unknown(0xE8);
        let _ = simulate(&EventType::KeyPress(dummy));
        let _ = simulate(&EventType::KeyRelease(dummy));
    });
}
