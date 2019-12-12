use tonic::transport::Channel;

use lib_protocol::for_client::client::ForClientClient;

use crate::{Config, Result};

pub struct System {
    config: Config,
    client: Option<ForClientClient<Channel>>,
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

    pub async fn client(&mut self) -> Result<&mut ForClientClient<Channel>> {
        if self.client.is_none() {
            let client = ForClientClient::connect(
                self.config.controller.address.clone()
            ).await?;

            self.client = Some(client);
        }

        Ok(self.client
            .as_mut()
            .unwrap())
    }
}