use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum ShellEngineError {
    //
}