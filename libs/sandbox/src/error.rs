use std::result;

use snafu::Snafu;

use crate::{LxdEngineError, ShellEngineError};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("The latest command returned a non-zero exit code"))]
    CommandFailed,

    #[snafu(display("LXD failure: {}", source))]
    LxdSandboxFailure {
        source: LxdEngineError,
    },

    #[snafu(display("Shell failure: {}", source))]
    ShellSandboxFailure {
        source: ShellEngineError,
    },
}

impl From<LxdEngineError> for Error {
    fn from(source: LxdEngineError) -> Self {
        Error::LxdSandboxFailure { source }
    }
}

impl From<ShellEngineError> for Error {
    fn from(source: ShellEngineError) -> Self {
        Error::ShellSandboxFailure { source }
    }
}
