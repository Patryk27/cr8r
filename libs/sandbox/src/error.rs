use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Command returned a non-zero exit code"))]
    CommandFailed,

    #[snafu(display("{}", source))]
    LxdError {
        source: lib_lxd::Error,
    },
}

impl From<lib_lxd::Error> for Error {
    fn from(source: lib_lxd::Error) -> Self {
        Error::LxdError { source }
    }
}