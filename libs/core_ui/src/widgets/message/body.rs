#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Info,
    Warn,
    Error,
    Success,
}
