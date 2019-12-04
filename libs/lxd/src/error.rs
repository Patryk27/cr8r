use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError {
        message: String,
    },

    LxdError {
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError { message } => write!(f, "{}", message),
            Error::LxdError { message } => write!(f, "{}", message),
        }
    }
}

impl error::Error for Error {
    //
}