use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
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
    message_list: VecDeque<LogMessage>,
}

impl Logger {
    pub fn new() -> Logger {
        Self {
            message_list: VecDeque::new(),
        }
    }

    pub fn debug(&mut self, message: impl AsRef<str>) {
        self.message_list.push_back(LogMessage { log_level: LogLevel::Debug, message: message.as_ref().to_string() })
    }

    pub fn error(&mut self, message: impl AsRef<str>) {
        self.message_list.push_back(LogMessage { log_level: LogLevel::Error, message: message.as_ref().to_string() })
    }

    pub fn first_message(&mut self) -> Option<LogMessage> {
        self.message_list.remove(0)
    }
}