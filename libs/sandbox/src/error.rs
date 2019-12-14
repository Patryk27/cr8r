use std::borrow::Cow;
use std::result;

use snafu::Snafu;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Command returned a non-zero exit code"))]
    CommandFailed,

    #[snafu(display("{}", source))]
    LxdError {
        source: lib_sandbox_lxd::Error,
    },

    #[snafu(display("Environmental variable `{}` is missing or contains invalid UTF-8 characters", name))]
    MissingEnvVariable {
        name: Cow<'static, str>,
    },

    #[snafu(display("Couldn't launch container: {}", source))]
    CouldntLaunchContainer {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },

    #[snafu(display("Couldn't forward SSH agent: {}", source))]
    CouldntForwardSshAgent {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },

    #[snafu(display("Couldn't wait for network: {}", source))]
    CouldntWaitForNetwork {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },

    #[snafu(display("Couldn't install toolchain: {}", source))]
    CouldntInstallToolchain {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
}

impl From<lib_sandbox_lxd::Error> for Error {
    fn from(source: lib_sandbox_lxd::Error) -> Self {
        Error::LxdError { source }
    }
}