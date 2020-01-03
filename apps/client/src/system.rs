use anyhow::Result;

use lib_interop::client::ControllerClient;

use crate::Config;

pub struct System {
    config: Config,
    client: Option<ControllerClient>,
}

impl System {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: None,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn client(&mut self) -> Result<&mut ControllerClient> {
        if self.client.is_none() {
            let controller = &self.config.controller;

            let client = ControllerClient::connect(
                controller.address.to_owned(),
                controller.secret.to_owned(),
            ).await?;

            self.client = Some(client);
        }

        Ok(self.client
            .as_mut()
            .unwrap())
    }
}