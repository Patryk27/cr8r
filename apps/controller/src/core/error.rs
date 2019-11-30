use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;
pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Failed to configure the controller: {:?}", source))]
    FailedToConfigure {
        source: Box<dyn std::error::Error>,
    },

    #[snafu(display("Failed to start the runner: {:?}", source))]
    FailedToStart {
        source: std::io::Error,
    },
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::FailedToConfigure {
            source: box err,
        }
    }
}