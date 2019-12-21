use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Field `{}` is missing", name))]
    Missing {
        name: &'static str,
    },

    #[snafu(display("Field `{}` contains invalid datetime: {}", name, source))]
    InvalidDateTime {
        name: &'static str,
        source: chrono::ParseError,
    },
}