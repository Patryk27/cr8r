use std::result;

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

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        tonic::Status::new(tonic::Code::Internal, err.msg)
    }
}
