use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;
pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Couldn't start the client: {:?}", source))]
    CouldntStart {
        source: Box<dyn std::error::Error>,
    },

    #[snafu(display("Couldn't connect to the controller: {}", source))]
    CouldntConnectToController {
        source: tonic::transport::Error,
    },

    #[snafu(display("Couldn't perform request to the controller: {}", source))]
    CouldntRequestController {
        source: tonic::Status,
    },

    #[snafu(display("Couldn't parse response: {}", source))]
    InteropError {
        source: lib_interop::Error,
    },
}

impl From<tonic::transport::Error> for Error {
    fn from(source: tonic::transport::Error) -> Self {
        Error::CouldntConnectToController { source }
    }
}

impl From<tonic::Status> for Error {
    fn from(source: tonic::Status) -> Self {
        Error::CouldntRequestController { source }
    }
}

impl From<lib_interop::Error> for Error {
    fn from(source: lib_interop::Error) -> Self {
        Error::InteropError { source }
    }
}