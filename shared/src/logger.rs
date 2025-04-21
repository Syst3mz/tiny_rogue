use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use core::cell::UnsafeCell;
use core::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    Debug,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct LogMessage {
    pub log_level: LogLevel,
    pub message: String,
}
impl Display for LogMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}]: {}", self.log_level, self.message)
    }
}

pub struct Logger {
    message_list: UnsafeCell<VecDeque<LogMessage>>,
}

impl Logger {
    pub const fn new() -> Logger {
        Self {
            message_list: UnsafeCell::new(VecDeque::new()),
        }
    }
    
    pub fn message(&self, level: LogLevel, message: impl AsRef<str>) {
        unsafe {
            (*self.message_list.get()).push_back(LogMessage {
                log_level: level,
                message: message.as_ref().to_string()
            })
        }
    }

    pub fn debug(&self, message: impl AsRef<str>) {
        self.message(LogLevel::Debug, message);       
    }

    pub fn error(&self, message: impl AsRef<str>) {
        self.message(LogLevel::Error, message);
    }

    pub fn next_message(&self) -> Option<LogMessage> {
        unsafe {
            (*self.message_list.get()).pop_front()
        }
    }
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}