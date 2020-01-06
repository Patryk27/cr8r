use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Info,
    Warn,
    Error,
    Success,
}

pub struct Message {
    ty: MessageType,
    header: String,
    content: String,
}

impl Message {
    pub fn new(ty: MessageType, header: String, content: String) -> Self {
        Self { ty, header, content }
    }

    pub fn info(header: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(MessageType::Info, header.into(), content.into())
    }

    pub fn warn(header: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(MessageType::Warn, header.into(), content.into())
    }

    pub fn error(header: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(MessageType::Error, header.into(), content.into())
    }

    pub fn success(header: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(MessageType::Success, header.into(), content.into())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        writeln!(f, "{}", self.header)?;

        for line in self.content.lines() {
            writeln!(f, "  {}", match self.ty {
                MessageType::Info => line.blue(),
                MessageType::Warn => line.yellow(),
                MessageType::Error => line.red(),
                MessageType::Success => line.green(),
            })?;
        }

        Ok(())
    }
}
