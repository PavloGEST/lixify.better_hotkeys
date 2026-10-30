use crate::window::{Window, ACTIVE_WINDOW};
use std::error::Error;

fn select_window_under_cursor(active: &mut Option<Window>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let selected_window = Window::under_cursor()?;

    if active
        .as_ref()
        .is_none_or(|window| !window.is_same_window(&selected_window))
    {
        *active = Some(selected_window);
    }

    Ok(())
}

pub fn _try_select_window_under_cursor() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)
}

pub fn _try_set_window_fullscreen() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)?;

    let result = active_window
        .as_mut()
        .expect("active window was initialized")
        .toggle();

    if let Err(error) = result {
        *active_window = None;
        return Err(error.into());
    }

    if active_window
        .as_ref()
        .is_some_and(|window| !window.is_fullscreen())
    {
        *active_window = None;
    }

    Ok(())
}

pub fn _try_quick_close_window() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)?;
    let window = active_window.as_mut().expect("window was selected");
    window.close()?;
    *active_window = None;
    Ok(())
}

pub fn _try_slide_to_desktop(number: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)?;
    let window = active_window.as_mut().expect("window was selected");
    window.slide_to_desktop(number)?;
    *active_window = None;
    Ok(())
}

pub fn _try_move_and_slide_to_desktop(number: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)?;
    let window = active_window.as_mut().expect("window was selected");
    window.move_to_desktop(number)?;
    window.slide_to_desktop(number)?;
    *active_window = None;
    Ok(())
}

pub fn _try_move_window_to_desktop(number: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut active_window = ACTIVE_WINDOW
        .lock()
        .map_err(|_| "Window state lock is poisoned")?;
    select_window_under_cursor(&mut active_window)?;
    let window = active_window.as_mut().expect("window was selected");
    window.move_to_desktop(number)?;
    *active_window = None;
    Ok(())
}


