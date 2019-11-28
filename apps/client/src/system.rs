use tonic::transport::Channel;

use lib_protocol::controller::client::ControllerClient;

use crate::Result;

pub use self::config::*;

mod config;

pub struct System {
    config: Config,
    client: Option<ControllerClient<Channel>>,
}

impl System {
    pub fn new() -> Result<Self> {
        let config = config::load()?;

        Ok(Self { config, client: None })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn client(&mut self) -> Result<&mut ControllerClient<Channel>> {
        if self.client.is_none() {
            self.client = Some(ControllerClient::connect(
                self.config.controller.address.clone()
            ).await?);
        }

        Ok(self.client
            .as_mut()
            .unwrap())
    }
}