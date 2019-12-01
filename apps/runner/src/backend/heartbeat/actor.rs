use std::time::Duration;

use log::*;
use tokio::timer;

use crate::core::SessionClient;

pub struct HeartbeatActor {
    session_client: SessionClient,
}

impl HeartbeatActor {
    pub fn new(session_client: SessionClient) -> Self {
        Self { session_client }
    }

    pub async fn start(mut self) {
        debug!("Heartbeat process started");

        loop {
            if let Err(err) = self.session_client.ping().await {
                error!("Failed to ping the controller: {:?}", err);
                error!("We'll try again in a moment");
            }

            timer::delay_for(Duration::from_secs(5)).await;
        }
    }
}