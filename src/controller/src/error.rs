use std::result;

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

    #[snafu(display("Could not start server: {:?}", source))]
    FailedToStart {
        source: std::io::Error,
    },
}