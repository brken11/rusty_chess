use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;

use crate::common::{Terminal, ThreadIdentifier};
use crate::time::time_format;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl From<u8> for LogLevel {
    fn from(val: u8) -> LogLevel {
        match val% 4 {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Warning,
            3 => LogLevel::Error,
            _ => unreachable!("n % 4 makes this branch unreachable."),
        }
    }
}

pub enum LogMessage {
    Message(ThreadIdentifier, LogLevel, String),
    Instruction(ThreadIdentifier, LogInstruction),
}

pub enum LogInstruction {
    SetLevel(LogLevel),
    SetOutput(LogOutput),
    SetPrintTimestamp(bool),
    SetTimestampDisplay(time_format::DisplayMode),
    SetPrintThreadIdentifier(bool),
    SetPrintLevel(bool),
    SetPrintMessage(bool),
    Shutdown,
}

pub enum LogOutput {
    None,
    // Exists so config can send Std without having to create channel,
    // otherwise Identical to Stdout while initialzing but treated as None for output matching
    InitStdout,
    Stdout(Sender<String>),
    File(std::fs::File),
}

enum TimeTracker {
    Date(time_format::DateMillis),
    Runtime(Instant),
}

pub struct LogThread {
    thread_identifier: ThreadIdentifier,
    channel_in: Receiver<LogMessage>,
    parent_thread: ThreadIdentifier,
    running: bool,
    print_timestamp: bool,
    print_thread_identifier: bool,
    print_level: bool,
    print_message: bool,
    level_threshold: LogLevel,
    out: LogOutput,
    date_millis: TimeTracker,
    date_display_mode: time_format::DisplayMode,
}

impl LogThread {
    pub fn new( parent_thread: ThreadIdentifier, ) -> ( LogThread, std::sync::mpsc::Sender<LogMessage>, ThreadIdentifier, ) {
        let (sender, channel_in) = std::sync::mpsc::channel();
        let thread_identifier = ThreadIdentifier::Log(ThreadIdentifier::generate_id());

        let terminal_sender = Terminal::get_sender();

        let date_millis = match time_format::DateMillis::new() {
            Ok(dt_mil) => TimeTracker::Date(dt_mil),
            Err(_) => TimeTracker::Runtime(Instant::now()),
        };

        (
            LogThread {
                thread_identifier,
                channel_in,
                parent_thread,
                running: false,
                print_timestamp: true,
                print_thread_identifier: true,
                print_level: true,
                print_message: true,
                out: LogOutput::Stdout(terminal_sender),
                level_threshold: LogLevel::Debug,
                date_millis,
                date_display_mode: time_format::DisplayMode::ISO8601,
            },
            sender,
            thread_identifier,
        )
    }
    pub fn from_id_channel_in_and_out( parent_thread: ThreadIdentifier, channel_in: Receiver<LogMessage>, out: LogOutput, ) -> (LogThread, ThreadIdentifier) {
        let thread_identifier = ThreadIdentifier::Log(ThreadIdentifier::generate_id());

        let date_millis = match time_format::DateMillis::new() {
            Ok(dt_mil) => TimeTracker::Date(dt_mil),
            Err(_) => TimeTracker::Runtime(Instant::now()),
        };
        (
            LogThread {
                thread_identifier,
                channel_in,
                parent_thread,
                running: false,
                print_timestamp: true,
                print_thread_identifier: true,
                print_level: true,
                print_message: true,
                out,
                level_threshold: LogLevel::Debug,
                date_millis,
                date_display_mode: time_format::DisplayMode::ISO8601,
            },
            thread_identifier,
        )
    }

    pub fn set_level_threshold(&mut self, level_threshold: LogLevel) {
        self.level_threshold = level_threshold;
    }
    pub fn set_output(&mut self, out: LogOutput) {
        match out {
            LogOutput::InitStdout => {
                let terminal = Terminal::get_sender();
                self.out = LogOutput::Stdout(terminal);
            }
            _ => self.out = out,
        }
    }
    pub fn set_print_timestamp(&mut self, print_timestamp: bool) {
        self.print_timestamp = print_timestamp;
    }
    pub fn set_timestamp_mode(&mut self, display_mode: time_format::DisplayMode) {
        self.date_display_mode = display_mode;
        match &mut self.date_millis {
            TimeTracker::Date(date_millis) => date_millis.set_display_mode(display_mode),
            _ => {}
        }
    }
    pub fn set_print_thread_identifier(&mut self, print_thread_identifier: bool) {
        self.print_thread_identifier = print_thread_identifier;
    }
    pub fn set_print_level(&mut self, print_level: bool) {
        self.print_level = print_level;
    }
    pub fn set_print_message(&mut self, print_message: bool) {
        self.print_message = print_message;
    }

    pub fn start(self) -> thread::JoinHandle<LogThread> {
        thread::spawn(move || self.run())
    }
    fn init_std_out(&mut self) {
        match self.out {
            LogOutput::InitStdout => {
                let terminal: Sender<String> = Terminal::get_sender();
                self.out = LogOutput::Stdout(terminal);
            }
            _ => {}
        }
    }
    fn run(mut self) -> LogThread {
        self.init_std_out();

        self.log_print(LogLevel::Info, "Log thread started".to_string());
        self.running = true;

        while self.running {
            if self.process_message().is_err() {
                break;
            }
        }

        self.log_print(LogLevel::Info, "Log thread stopped".to_string());
        self.shutdown(&self.thread_identifier.clone());
        self
    }
    fn process_message(&mut self) -> Result<(), ()> {
        match self.channel_in.recv() {
            Ok(LogMessage::Message(thread_identifier, level, message)) => {
                if level >= self.level_threshold {
                    self.print(&thread_identifier, level, message);
                }
                Ok(())
            }
            Ok(LogMessage::Instruction(thread_identifier, log_instruction)) => {
                if thread_identifier == self.parent_thread {
                    match log_instruction {
                        LogInstruction::SetLevel(level) => self.set_level_threshold(level),
                        LogInstruction::SetOutput(out) => self.set_output(out),
                        LogInstruction::SetPrintTimestamp(print_timestamp) => {
                            self.set_print_timestamp(print_timestamp)
                        }
                        LogInstruction::SetTimestampDisplay(display_mode) => {
                            self.set_timestamp_mode(display_mode)
                        }
                        LogInstruction::SetPrintThreadIdentifier(print_thread_identifier) => {
                            self.set_print_thread_identifier(print_thread_identifier)
                        }
                        LogInstruction::SetPrintLevel(print_level) => {
                            self.set_print_level(print_level)
                        }
                        LogInstruction::SetPrintMessage(print_message) => {
                            self.set_print_message(print_message)
                        }
                        LogInstruction::Shutdown => self.shutdown(&thread_identifier),
                    }
                }
                Ok(())
            }
            Err(_) => {
                println!("{}: Log channel closed", self.thread_identifier);
                Err(())
            }
        }
    }

    fn compose_message(&self, thread_identifier: &ThreadIdentifier, log_level: LogLevel, message: String, ) -> String {
        let mut composed_message = String::new();
        if self.print_timestamp {
            match &self.date_millis {
                TimeTracker::Date(date_millis) => {
                    composed_message.push_str(&format!("Time[{}] ", date_millis));
                }
                TimeTracker::Runtime(instant) =>
                    composed_message.push_str(&format!("Time[{:10}] ", instant.elapsed().as_millis())),
            };
        }
        if self.print_thread_identifier {
            composed_message.push_str(&format!("ThreadId[{}] ", thread_identifier));
        }
        if self.print_level {
            composed_message.push_str(&format!("Level[{}] ", log_level));
        }
        if self.print_message {
            composed_message.push_str(&format!(":{}", message));
        }
        //composed_message.push('\n');

        composed_message
    }
    fn log_print(&mut self, log_level: LogLevel, message: String) {
        let thread_identifier = self.thread_identifier.clone();
        self.print(&thread_identifier, log_level, message);
    }
    fn print(&mut self, thread_identifier: &ThreadIdentifier, log_level: LogLevel, message: String, ) {
        if self.print_timestamp { match &mut self.date_millis {
            TimeTracker::Date(date_millis) => date_millis.update(),
            _ => {}
        }}

        match &self.out {
            LogOutput::Stdout(sender) => {
                if sender
                    .send(self.compose_message(thread_identifier, log_level, message))
                    .is_err()
                {
                    self.out = LogOutput::None
                }
            }
            LogOutput::File(file) => {
                todo!() // Implement file logic
            }
            LogOutput::InitStdout => {
                self.init_std_out();
                self.print(thread_identifier, log_level, message);
            }
            LogOutput::None => {}
        }
    }

    fn shutdown(&mut self, thread_identifier: &ThreadIdentifier) {
        if self.running {
            self.running = false;

            self.print(
                thread_identifier,
                LogLevel::Info,
                "Shutdown order received, emptying queue".to_string(),
            );
        }
        //Flush queue
        while self.process_message().is_ok() {}
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error"),
        }
    }
}
