use anyhow::*;

use lib_interop::client::ControllerClient;

use crate::app::AppConfig;

pub struct AppContext {
    config: AppConfig,
    client: Option<ControllerClient>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            client: None,
        }
    }

    pub fn config(&self) -> &AppConfig {
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