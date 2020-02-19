use log::*;
use tokio::task::spawn;
use tokio::time::{delay_for, Duration, timeout};

use crate::rpc::Session;

pub struct HeartbeatSyncer;

impl HeartbeatSyncer {
    pub fn new(session: Session) {
        spawn(async move {
            let mut client = session
                .conn()
                .runners();

            loop {
                trace!("Syncing heartbeat");

                let sync = client.sync_heartbeat(
                    session.runner_id(),
                );

                let result = timeout(
                    Duration::from_secs(5),
                    sync,
                ).await;

                match result {
                    Ok(Ok(())) => {
                        //
                    }

                    Ok(Err(err)) => {
                        error!("Could not sync heartbeat with the controller: {:?}", err);
                        error!("We'll try again in a few seconds");
                    }

                    Err(_) => {
                        error!("Could not sync heartbeat with the controller: Timed out");
                        error!("We'll try again in a few seconds");
                    }
                }

                delay_for(Duration::from_secs(5)).await;
            }
        });
    }
}