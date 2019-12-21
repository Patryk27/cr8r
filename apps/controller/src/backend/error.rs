use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl From<&'_ str> for Error {
    fn from(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl From<lib_compiler::Error> for Error {
    fn from(err: lib_compiler::Error) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

impl From<lib_interop::Error> for Error {
    fn from(err: lib_interop::Error) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

// This one's required, because `tonic`'s `Status::internal()` function requires an `Into<String>`
impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
