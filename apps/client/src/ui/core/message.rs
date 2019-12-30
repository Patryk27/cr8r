use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Info,
    Warn,
    Error,
}

pub struct Message {
    ty: MessageType,
    msg: String,
}

impl Message {
    pub fn new(ty: MessageType, msg: impl Into<String>) -> Self {
        Self { ty, msg: msg.into() }
    }

    pub fn info(msg: impl Into<String>) -> Self {
        Self::new(MessageType::Info, msg)
    }

    pub fn warn(msg: impl Into<String>) -> Self {
        Self::new(MessageType::Warn, msg)
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self::new(MessageType::Error, msg)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @todo
        unimplemented!()
    }
}