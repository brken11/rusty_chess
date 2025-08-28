mod backend_win32;
mod backend_x11;

use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

use crate::board::Square;
use crate::common::ThreadIdentifier;
use crate::Board;
use crate::LogLevel;
use crate::LogMessage;
use crate::common::common_lib::Log;

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

pub enum UserInput {
    PauseMenu,
    QuitGame,
}

pub struct UIManager {
    thread_identifier: ThreadIdentifier,
    ui_type: UIType,
    ui_state: UIState,
    log_channel: Option<mpsc::Sender<LogMessage>>,
}

trait GraphicsBackend {
    fn new() -> Self;
    fn start(self) -> thread::JoinHandle<Self>
    where
        Self: Sized;
    fn stop(self);
}
pub enum UiUpdate {
    ParseMove(Square, Square),
    ParseUserInput(String),
    ShuttingDown,
}
pub enum UiMessage {
    BoardUpdate(Board),
}

impl Log for UIManager {
    fn get_thread_id(&self) -> ThreadIdentifier {
        self.thread_identifier
    }
}
impl UIManager {
    pub fn new(log_channel: Option<mpsc::Sender<LogMessage>>) -> Self {
        let thread_identifier = ThreadIdentifier::UI(ThreadIdentifier::generate_id());
        let new_manager = UIManager {
            thread_identifier,
            ui_type: UIType::Terminal,
            ui_state: UIState::Menu,
            log_channel,
        };
        new_manager.log(LogLevel::Info, "Initializing UIManager".to_string());
        new_manager
    }

    pub fn start(self) -> JoinHandle<UIManager> {
        self.log(LogLevel::Debug, "UIManager.start()".to_string());
        thread::spawn(move || self.run())
    }
    fn run(mut self) -> UIManager {
        self.log(LogLevel::Info, "UIManager thread started".to_string());

        loop {
            // test
            for i in 0..4 {
                thread::sleep(std::time::Duration::new(1, 0));
                self.log(LogLevel::Debug, format!("UI loop test: {}", i));
            }
            thread::sleep(std::time::Duration::new(10, 0));
            break;
        }

        self
    }

    pub fn set_ui_type(&mut self, ui_type: UIType) {
        self.ui_type = ui_type;
    }

    //fn log(&mut self, message: String, log_level: LogLevel) {
    //    if let Some(log) = &self.log_channel {
    //        match log.send(LogMessage::Message(
    //            self.thread_identifier,
    //            log_level,
    //            message,
    //        )) {
    //            Ok(_) => {}
    //            Err(_) => self.log_channel = None,
    //        }
    ////    }
    //}
}
