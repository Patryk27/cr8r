use tonic::transport::Channel;

use lib_protocol::client::client::ClientClient;

use crate::{Config, Result};

pub struct System {
    config: Config,
    client: Option<ClientClient<Channel>>,
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

    pub async fn client(&mut self) -> Result<&mut ClientClient<Channel>> {
        if self.client.is_none() {
            let address = self.config.controller.address.clone();

            self.client = Some(
                ClientClient::connect(address).await?,
            );
        }

        Ok(self.client
            .as_mut()
            .unwrap())
    }
}