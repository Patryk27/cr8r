use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Couldn't find LXD's client executable (e.g. `/snap/bin/lxc`) - please ensure you have LXD installed"))]
    ClientNotFound,

    #[snafu(display("Couldn't parse LXD's response: {}", source))]
    ClientReturnedGarbage {
        source: serde_json::error::Error,
    },

    #[snafu(display("Couldn't start command: {}", source))]
    CommandNotStarted {
        source: std::io::Error,
    },

    #[snafu(display("Command returned a non-zero exit code"))]
    CommandFailed,

    #[snafu(display("`{}` is not a valid identifier - only alphanumeric characters are allowed", ident))]
    InvalidIdentifier {
        ident: String,
    },
}