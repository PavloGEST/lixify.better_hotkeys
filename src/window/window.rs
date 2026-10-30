use std::error::Error;
use std::sync::Mutex;
use winsafe::{self as w, co, msg};

pub struct SavedWindowState {
    style: co::WS,
    placement: w::WINDOWPLACEMENT,
}

pub struct Window {
    hwnd: w::HWND,
    saved_state: Option<SavedWindowState>,
}

impl Window {
    pub fn under_cursor() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let cursor = w::GetCursorPos()?;
        let hwnd = w::HWND::WindowFromPoint(cursor)
            .and_then(|hwnd| hwnd.GetAncestor(co::GA::ROOT))
            .ok_or("No window under cursor")?;
        Ok(Self {
            hwnd,
            saved_state: None,
        })
    }

    pub fn is_same_window(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
    }

    pub fn enter_fullscreen(&mut self) -> w::SysResult<()> {
        let original_style = self.hwnd.style();
        let mut placement = w::WINDOWPLACEMENT::default();
        self.hwnd.GetWindowPlacement(&mut placement)?;

        let monitor = self.hwnd.MonitorFromWindow(co::MONITOR::DEFAULTTONEAREST);
        let rect = monitor.GetMonitorInfo()?.rcMonitor;
        let new_style =
            unsafe { co::WS::from_raw(original_style.raw() & !co::WS::OVERLAPPEDWINDOW.raw()) };

        self.hwnd.set_style(new_style);
        self.hwnd.SetWindowPos(
            w::HwndPlace::None,
            w::POINT::with(rect.left, rect.top),
            w::SIZE::with(rect.right - rect.left, rect.bottom - rect.top),
            co::SWP::NOZORDER | co::SWP::FRAMECHANGED | co::SWP::SHOWWINDOW,
        )?;

        self.saved_state = Some(SavedWindowState {
            style: original_style,
            placement,
        });
        Ok(())
    }

    pub fn exit_fullscreen(&mut self) -> w::SysResult<()> {
        let saved_state = self.saved_state.as_ref().ok_or(co::ERROR::INVALID_STATE)?;

        self.hwnd.set_style(saved_state.style);
        self.hwnd.SetWindowPos(
            w::HwndPlace::None,
            w::POINT::default(),
            w::SIZE::default(),
            co::SWP::NOZORDER | co::SWP::NOMOVE | co::SWP::NOSIZE | co::SWP::FRAMECHANGED,
        )?;
        self.hwnd.SetWindowPlacement(&saved_state.placement)?;
        self.saved_state = None;
        Ok(())
    }

    pub fn toggle(&mut self) -> w::SysResult<()> {
        if self.saved_state.is_some() {
            self.exit_fullscreen()
        } else {
            self.enter_fullscreen()
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.saved_state.is_some()
    }

    pub fn close(&mut self) -> w::SysResult<()> {
        unsafe { self.hwnd.PostMessage(msg::WmClose {}) }
    }


    pub fn start_drag(&self) -> Result<Option<WindowDrag>, Box<dyn Error + Send + Sync>> {
        let class = self.hwnd.GetClassName()?;
        if matches!(
            class.as_str(),
            "Shell_TrayWnd" | "Shell_SecondaryTrayWnd" | "Progman" | "WorkerW"
        ) {
            return Ok(None);
        }

        let cursor = w::GetCursorPos()?;
        let mut rect = self.hwnd.GetWindowRect()?;

        if self.hwnd.IsZoomed() {
            let ratio = f64::from(cursor.x - rect.left) / f64::from((rect.right - rect.left).max(1));
            self.hwnd.ShowWindow(co::SW::RESTORE);
            rect = self.hwnd.GetWindowRect()?;
            let width = rect.right - rect.left;
            let shifted = cursor.x - (f64::from(width) * ratio) as i32;
            self.hwnd.SetWindowPos(
                w::HwndPlace::None,
                w::POINT::with(shifted, rect.top),
                w::SIZE::default(),
                co::SWP::NOSIZE | co::SWP::NOZORDER,
            )?;
            rect = self.hwnd.GetWindowRect()?;
        }

        Ok(Some(WindowDrag {
            hwnd: unsafe { w::HWND::from_ptr(self.hwnd.ptr()) },
            cursor_start: cursor,
            origin: w::POINT::with(rect.left, rect.top),
        }))
    }

    pub fn slide_to_desktop(&self, number: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
        let desktop = Self::desktop_by_number(number)?;
        winvd::switch_desktop(desktop).map_err(|e| format!("{e:?}"))?;
        Ok(())
    }

    pub fn move_to_desktop(&self, number: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
        let desktop = Self::desktop_by_number(number)?;
        let hwnd = windows::Win32::Foundation::HWND(self.hwnd.ptr());
        winvd::move_window_to_desktop(desktop, &hwnd).map_err(|e| format!("{e:?}"))?;
        Ok(())
    }

    fn desktop_by_number(number: u32) -> Result<winvd::Desktop, Box<dyn Error + Send + Sync>> {
        let index = number.checked_sub(1).ok_or("Desktop numbers start at 1")?;
        let desktops = winvd::get_desktops().map_err(|e| format!("{e:?}"))?;
        desktops
            .into_iter()
            .nth(index as usize)
            .ok_or_else(|| format!("Desktop {number} does not exist").into())
    }
}

pub struct WindowDrag {
    hwnd: w::HWND,
    cursor_start: w::POINT,
    origin: w::POINT,
}

impl WindowDrag {
    pub fn update(&self) -> w::SysResult<()> {
        let cursor = w::GetCursorPos()?;
        self.hwnd.SetWindowPos(
            w::HwndPlace::None,
            w::POINT::with(
                self.origin.x + cursor.x - self.cursor_start.x,
                self.origin.y + cursor.y - self.cursor_start.y,
            ),
            w::SIZE::default(),
            co::SWP::NOSIZE | co::SWP::NOZORDER | co::SWP::NOACTIVATE,
        )
    }
}

pub(crate) static ACTIVE_WINDOW: Mutex<Option<Window>> = Mutex::new(None);