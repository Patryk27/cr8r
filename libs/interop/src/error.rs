use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Field `{}` is missing", field))]
    Missing {
        field: &'static str,
    },

    #[snafu(display("Field `{}` contains invalid datetime: {}", field, source))]
    InvalidDateTime {
        field: &'static str,
        source: chrono::ParseError,
    },
}