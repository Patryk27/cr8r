use std::path::PathBuf;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum ShellEngineError {
    #[snafu(display("Root directory (`{}`) is not accessible: {}", root.display(), source))]
    RootDirectoryInaccessible {
        root: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Couldn't start command: {}", source))]
    CommandNotStarted {
        source: std::io::Error,
    },

    #[snafu(display("Command returned a non-zero exit code"))]
    CommandFailed,
}