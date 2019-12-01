use std::time::Duration;

use log::*;
use tokio::timer;

use crate::core::{Result, Session};

pub struct RunnerActor {
    session: Session,
}

impl RunnerActor {
    pub fn new(session: Session) -> Self {
        Self { session }
    }

    pub async fn start(mut self) -> Result<()> {
        let assignment = loop {
            debug!("Polling controller for an assignment");

            if let Some(assignment) = self.session.request_assignment().await? {
                info!("We've been assigned an experiment!");
                info!("-> experiment id: {:?}", assignment.experiment_id);

                break assignment;
            }

            timer::delay_for(Duration::from_secs(1)).await;
        };

        Ok(())
    }
}