use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum Error {
    #[snafu(display("Could not open configuration file: {:?}", source))]
    FailedToOpenConfig {
        source: std::io::Error,
    },

    #[snafu(display("Could not parse configuration file: {:?}", source))]
    FailedToParseConfig {
        source: serde_yaml::Error,
    },

    #[snafu(display("Controller rejected our registration request: {:?}", source))]
    FailedToRegister {
        source: Box<dyn std::error::Error>,
    },
}