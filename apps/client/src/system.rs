use crate::Result;

pub use self::{
    config::*,
    connector::*,
};

mod config;
mod connector;

pub struct System {
    config: Config,
    connector: Connector,
}

impl System {
    pub fn new() -> Result<Self> {
        let config = config::load()?;

        let connector = Connector::new(
            config.controller.address.clone(),
            config.controller.secret.clone(),
        )?;

        Ok(Self { config, connector })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn connector(&self) -> &Connector {
        &self.connector
    }
}