use std::time::Duration;

use log::*;
use tokio::timer;

use crate::core::SessionClient;

pub struct SystemHeartbeaterActor {
    client: SessionClient,
}

impl SystemHeartbeaterActor {
    pub fn new(client: SessionClient) -> Self {
        Self { client }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        loop {
            if let Err(err) = self.client.ping().await {
                error!("Failed to ping the controller: {:?}", err);
                error!("We'll try again in a moment");
            }

            timer::delay_for(Duration::from_secs(60)).await;
        }
    }
}