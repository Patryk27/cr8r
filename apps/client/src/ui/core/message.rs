use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Info,
    Warn,
    Error,
}

pub struct Message<'a> {
    ty: MessageType,
    message: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(ty: MessageType, message: &'a str) -> Self {
        Self { ty, message }
    }

    pub fn info(message: &'a str) -> Self {
        Self::new(MessageType::Info, message)
    }

    pub fn warn(message: &'a str) -> Self {
        Self::new(MessageType::Warn, message)
    }

    pub fn error(message: &'a str) -> Self {
        Self::new(MessageType::Error, message)
    }
}

impl fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @todo
        unimplemented!()
    }
}