use snafu::Snafu;

use crate::Error;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum LxdEngineError {
    #[snafu(display("{}", source))]
    ClientError {
        source: lib_lxd::Error,
    },

    #[snafu(display("Host environmental variable `{}` couldn't be read: {}", key, source))]
    HostEnvVarError {
        key: String,
        source: std::env::VarError,
    },
}

impl From<lib_lxd::Error> for Error {
    fn from(source: lib_lxd::Error) -> Self {
        Error::LxdError {
            source: LxdEngineError::ClientError {
                source,
            }
        }
    }
}