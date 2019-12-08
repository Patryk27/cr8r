use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Could not find LXD's client executable (e.g. `/snap/bin/lxc`) - please ensure you have installed LXD"))]
    FailedToAutodetectClient,

    #[snafu(display("`{}` is not a valid identifier - please use only alphanumeric characters", ident))]
    InvalidIdentifier {
        ident: String,
    },
}