mod backend_x11;
mod backend_win32;

use std::thread;

#[cfg(target_os = "linux")]
type CurrentBackend = backend_x11::X11Backend;
#[cfg(target_os = "windows")]
type CurrentBackend = backend_win32::Win32Backend;

pub enum UIType {
    Terminal,
    GUI,
    Web,
}

pub enum UIState {
    Menu,
    Game,
}

pub struct UIManager {
    ui_type: UIType,
    ui_state: UIState,
}

trait GraphicsBackend {
    fn new() -> Self;
    fn start(self) ->  thread::JoinHandle<Self> where Self: Sized;
    fn stop(self);
}

