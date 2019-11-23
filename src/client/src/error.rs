use std::result;

use semver::Version;
use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Could not open configuration file: {:?}", source))]
    FailedToOpenConfig {
        source: std::io::Error,
    },

    #[snafu(display("Could not parse configuration file: {:?}", source))]
    FailedToParseConfig {
        source: serde_yaml::Error,
    },

    #[snafu(display("Controller is not compatible with this client (protocol version mismatch - controller uses `{}`, we use `{}`)", controller_version, lib_protocol::version()))]
    ProtocolVersionMismatch {
        controller_version: Version,
    },

    #[snafu(display("Could not perform request to the controller: {:?}", source))]
    FailedToPerformRequest {
        source: reqwest::Error,
    },

    #[snafu(display("Could not process response from the controller: {:?}", source))]
    FailedToProcessResponse {
        source: Box<dyn std::error::Error>,
    },
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::FailedToPerformRequest { source }
    }
}