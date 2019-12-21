use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;
pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Couldn't start the controller: {:?}", source))]
    CouldntStart {
        source: Box<dyn std::error::Error>,
    },
}

impl From<std::net::AddrParseError> for Error {
    fn from(source: std::net::AddrParseError) -> Self {
        Error::CouldntStart {
            source: box source,
        }
    }
}
