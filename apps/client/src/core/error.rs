use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;
pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Failed to start the client: {:?}", source))]
    FailedToStart {
        source: Box<dyn std::error::Error>,
    },

    #[snafu(display("Failed to connect to the controller: {}", source))]
    FailedToConnectToController {
        source: tonic::transport::Error,
    },

    #[snafu(display("Failed to perform request to the controller: {}", source))]
    FailedToRequestController {
        source: tonic::Status,
    },
}

impl From<tonic::transport::Error> for Error {
    fn from(source: tonic::transport::Error) -> Self {
        Error::FailedToConnectToController { source }
    }
}

impl From<tonic::Status> for Error {
    fn from(source: tonic::Status) -> Self {
        Error::FailedToRequestController { source }
    }
}