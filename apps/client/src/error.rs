use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;
pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Failed to configure the client: {:?}", source))]
    FailedToConfigure {
        source: Box<dyn std::error::Error>,
    },

    #[snafu(display("Failed to perform request to the controller: {:?}", source))]
    FailedToPerformRequest {
        source: reqwest::Error,
    },

    #[snafu(display("Failed to process response from the controller: {:?}", source))]
    FailedToProcessResponse {
        source: Box<dyn std::error::Error>,
    },
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::FailedToPerformRequest { source }
    }
}