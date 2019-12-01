use std::time::Duration;

use log::*;
use tokio::timer;

use crate::core::Session;

pub struct HeartbeatActor {
    session: Session,
}

impl HeartbeatActor {
    pub fn new(session: Session) -> Self {
        Self { session }
    }

    pub async fn start(mut self) {
        debug!("Heartbeat process started");

        loop {
            if let Err(err) = self.session.ping().await {
                error!("Failed to ping the controller: {:?}", err);
                error!("We'll try again in a moment");
            }

            timer::delay_for(Duration::from_secs(5)).await;
        }
    }
}