use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum LxdEngineError {
    ClientError {
        source: lib_lxd::Error,
    },

    EnvVarNotFound {
        // @todo
    },
}