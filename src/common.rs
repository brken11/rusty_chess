use std::thread;

use std::collections::HashSet;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{OnceLock,Mutex};
use std::sync::atomic::{AtomicU32,Ordering};

use std::time::Instant;

use crate::log::{LogMessage,LogLevel};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ThreadIdentifier {
    Main(ThreadIdHash),
    Log(ThreadIdHash),
    Network(ThreadIdHash),
    Game(ThreadIdHash),
    UI(ThreadIdHash),
    Terminal(ThreadIdHash),
    GUI(ThreadIdHash),
    Other(ThreadIdHash, &'static str),
}

impl ThreadIdentifier {
    pub(crate) fn generate_id() -> ThreadIdHash {
        ThreadIdHash::new()
    }
}

pub type ThreadIdHash = u128;
type ThreadIdHashKey = u32;
static USED_THREAD_HASH_KEYS: OnceLock<Mutex<HashSet<ThreadIdHash>>> = OnceLock::new();
static KEY_COUNTER: AtomicU32 = AtomicU32::new(0);

pub trait ThreadIdHashExt {
    const UPPER_96: u128;
    fn new() -> Self;
    fn to_hash_key(&self) -> ThreadIdHashKey;
    fn upper_96(&self) -> u128;
    fn new_key() -> ThreadIdHashKey;
}
impl ThreadIdHashExt for ThreadIdHash {
    const UPPER_96: u128 = 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_00000000;
    fn new() -> Self {
        let mut value;
        let mut offset = 32;
        let key = ThreadIdHash::new_key();
        let mutex = USED_THREAD_HASH_KEYS.get_or_init(|| Mutex::new(HashSet::new()));
        let start = Instant::now();

        loop {
            {//Scope to drop lock
                let mut key_hash_set = mutex.lock().unwrap();

                value = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or(Instant::now().duration_since(start))
                        .as_nanos();
                value = 19*value*value*value - 17*value*value + (offset as u128)*value; // "Randomize"
                value = value.rotate_left(offset); //Default put upper 32 as lowest 32
                value &= ThreadIdHash::UPPER_96; // Keep upper 96, preserving the most volitile
                                                 // bits and placing original upper32 as lowest 32.
                value |= key as u128; // merge key with psuedo random value

                if key_hash_set.insert(value.upper_96()) {
                    break;
                }
                offset += 1;
            }// Drop lock
        }

        value
    }
    fn upper_96(&self) -> ThreadIdHash {
        self & ThreadIdHash::UPPER_96
    }
    fn to_hash_key(&self) -> ThreadIdHashKey {
        (self % ( 1 << 32)) as ThreadIdHashKey
    }
    fn new_key() -> ThreadIdHashKey {
        KEY_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

impl std::fmt::Display for ThreadIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ThreadIdentifier::Other(thread_hash, name) => write!(f, "Other[{}:{}]", name, thread_hash.to_hash_key()),
            ThreadIdentifier::Main(thread_hash) => write!(f, "Main[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::Log(thread_hash) => write!(f, "Log[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::Network(thread_hash) => write!(f, "Network[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::Game(thread_hash) => write!(f, "Game[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::UI(thread_hash) => write!(f, "UI[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::Terminal(thread_hash) => write!(f, "Terminal[{}]", thread_hash.to_hash_key()),
            ThreadIdentifier::GUI(thread_hash) => write!(f, "GUI[{}]", thread_hash.to_hash_key()),
        }
    }
}

static TERMINAL: OnceLock<Terminal> = OnceLock::new();

pub struct Terminal {
    sender: Sender<String>,
}
impl Terminal {
    fn new() -> (Terminal, Receiver<String>) {
        let (sender, receiver) = channel();

        (Terminal { sender }, receiver)
    }
    pub fn get_sender() -> Sender<String> {
        let terminal = TERMINAL.get_or_init(Terminal::start);

        terminal.sender.clone()
    }

    fn start() -> Terminal {
        let (terminal, receiver) = Terminal::new();
        thread::spawn(move || Terminal::run(receiver));
        terminal
    }
    fn run(receiver: Receiver<String>) {
        for message in receiver {
            println!("{}", message);
        }
    }
}


pub mod common_lib {
    use super::{Sender,LogMessage,LogLevel,ThreadIdentifier};

    use std::thread::JoinHandle;
    use std::sync::{RwLock};
    use std::sync::atomic::{AtomicU8,Ordering};

    use crate::log::{LogThread,LogOutput};

    static LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);
    static LOG_SENDER: RwLock<Option<Sender<LogMessage>>> = RwLock::new(None);

    fn set_log_level(log_level: LogLevel) {
        LOG_LEVEL.store(log_level as u8, Ordering::Relaxed);
    }
    pub fn set_log_thread_log_level(thread_id: ThreadIdentifier, log_level: LogLevel) {
        set_log_level(log_level);
        if let Some(log_sender) = get_log_sender() {
            let _ = log_sender.send(LogMessage::Instruction(thread_id, crate::log::LogInstruction::SetLevel(log_level)));
        }
    }
    pub fn get_log_level() -> LogLevel {
        LogLevel::from(LOG_LEVEL.load(Ordering::Relaxed))
    }
    pub fn get_log_sender() -> Option<Sender<LogMessage>> {
        match LOG_SENDER.read() {
            Ok(sender) => sender.clone(),
            Err(_) => None,
        }
    }
    fn set_sender(new_sender: Option<Sender<LogMessage>>) {
        if let Ok(mut sender) = LOG_SENDER.write() {
            *sender = new_sender;
        }
    }

    pub trait Log {
        fn get_thread_id(&self) -> ThreadIdentifier;
        //fn get_log_channel(&self) -> &mut Option<Sender<LogMessage>>;

        fn log(&self, level: LogLevel, message: String){
            if level < get_log_level() {return}

            let log_channel = get_log_sender();
            let sender = if let Some(lc) = log_channel {
                lc
            } else {
                return;
            };

            let result = sender.send(LogMessage::Message(
                    self.get_thread_id(),
                    level,
                    message
            ));
            //if result.is_err() { *log_channel = None; }
            if result.is_err() { //Log is poisoned or closed
                set_sender(None);
            }
        }
    }

    pub fn init_logger(thread_id: ThreadIdentifier, log_output: LogOutput) -> (JoinHandle<LogThread>, Sender<LogMessage>) {
        let (mut logger, log_sender, _thread_identifier) = LogThread::new(thread_id);

        logger.set_output(log_output);
        match LOG_SENDER.write() {
            Ok(mut writer) => {
                *writer = Some(log_sender.clone());
            },
            Err(e) => {
                eprintln!("Error occured setting sender: {}", e);
            }
        }
        (logger.start(), log_sender)
    }
}
