use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::thread;
use crate::common::ThreadIdentifier;
use std::time::SystemTime;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub enum LogMessage {
    Message(ThreadIdentifier, LogLevel, String),
    Instruction(ThreadIdentifier, LogInstruction),
}

pub enum LogInstruction {
    SetLevel(LogLevel),
    SetOutput(LogOutput),
    SetPrintTimestamp(bool),
    SetPrintThreadIdentifier(bool),
    SetPrintLevel(bool),
    SetPrintMessage(bool),
    Shutdown,
}

pub enum LogOutput {
    Stdout,
    File(std::fs::File),
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
}

impl LogThread {
    pub fn new(parent_thread: ThreadIdentifier) -> (LogThread, std::sync::mpsc::Sender<LogMessage>, ThreadIdentifier)  {
        let (sender, channel_in) = std::sync::mpsc::channel();
        let thread_identifier = ThreadIdentifier::Log(ThreadIdentifier::generate_id());
        (LogThread {
            thread_identifier,
            channel_in,
            parent_thread,
            running: false,
            print_timestamp: true,
            print_thread_identifier: true,
            print_level: true,
            print_message: true,
            out: LogOutput::Stdout,
            level_threshold: LogLevel::Debug,
        }, sender, thread_identifier)
    }
    pub fn from_id_channel_in_and_out(parent_thread: ThreadIdentifier, channel_in: Receiver<LogMessage>, out: LogOutput) -> (LogThread, ThreadIdentifier) {
        let thread_identifier = ThreadIdentifier::Log(ThreadIdentifier::generate_id());
        (LogThread {
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
        }, thread_identifier)
    }

    pub fn set_level_threshold(&mut self, level_threshold: LogLevel) {
        self.level_threshold = level_threshold;
    }
    pub fn set_output(&mut self, out: LogOutput) {
        self.out = out;
    }
    pub fn set_print_timestamp(&mut self, print_timestamp: bool) {
        self.print_timestamp = print_timestamp;
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
    fn run(mut self) -> LogThread {
        self.print(&self.thread_identifier, LogLevel::Info, "Log thread started".to_string());
        self.running = true;

        while self.running {if self.process_message().is_err(){break;}}

        self.print(&self.thread_identifier, LogLevel::Info, "Log thread stopped".to_string());
        self.shutdown(&self.thread_identifier.clone());
        self
    }
    fn process_message(&mut self) -> Result<(), ()>{
        match self.channel_in.recv() {
            Ok(LogMessage::Message(thread_identifier, level, message)) => {
                if level >= self.level_threshold {self.print(&thread_identifier, level, message);}
                Ok(())
            },
            Ok(LogMessage::Instruction(thread_identifier, log_instruction)) => {
                if thread_identifier == self.parent_thread {
                    match log_instruction {
                        LogInstruction::SetLevel(level) => self.set_level_threshold(level),
                        LogInstruction::SetOutput(out) => self.set_output(out),
                        LogInstruction::SetPrintTimestamp(print_timestamp) => self.set_print_timestamp(print_timestamp),
                        LogInstruction::SetPrintThreadIdentifier(print_thread_identifier) => self.set_print_thread_identifier(print_thread_identifier),
                        LogInstruction::SetPrintLevel(print_level) => self.set_print_level(print_level),
                        LogInstruction::SetPrintMessage(print_message) => self.set_print_message(print_message),
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

    fn compose_message(&self, thread_identifier: &ThreadIdentifier, log_level: LogLevel, message: String) -> String {
        let mut composed_message = String::new();
        if self.print_timestamp {
            let system_time = SystemTime::now();
            composed_message.push_str(&format!("Time[{:?}] ", system_time));
        }
        if self.print_thread_identifier {
            composed_message.push_str(&format!("ThreadId[{}]", thread_identifier));
        }
        if self.print_level {
            composed_message.push_str(&format!("Level[{}]", log_level));
        }
        if self.print_message {
            composed_message.push_str(&format!(" :{}", message));
        }
        composed_message.push('\n');

        composed_message
    }
    fn print(&self, thread_identifier: &ThreadIdentifier, log_level: LogLevel, message: String){
        match &self.out {
            LogOutput::Stdout => print!("{}", self.compose_message(thread_identifier, log_level, message)),
            LogOutput::File(file) => {
                todo!() // Implement file logic
            }
        }
    }

    fn shutdown(&mut self, thread_identifier: &ThreadIdentifier) {
        if self.running {
            self.running = false;

            self.print(thread_identifier, LogLevel::Info, "Shutdown order received, emptying queue".to_string());
        }
        //Flush queue
        while self.process_message().is_ok() {}
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error"),
        }
    }
}