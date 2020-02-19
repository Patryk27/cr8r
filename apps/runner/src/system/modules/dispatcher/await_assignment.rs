use log::*;
use tokio::time::{delay_for, Duration};

use lib_interop::models::DExperimentId;

use crate::system::Dispatcher;

impl Dispatcher {
    pub(super) async fn await_assignment(&mut self) -> DExperimentId {
        loop {
            trace!("Polling controller for a new assignment");

            let assignment = self.session
                .conn()
                .assignments()
                .prepare(self.session.runner_id()).await;

            match assignment {
                Ok(Some(experiment_id)) => {
                    info!("We've been assigned experiment [id={}]", experiment_id);

                    return experiment_id;
                }

                Ok(None) => {
                    delay_for(Duration::from_secs(1)).await;
                }

                Err(err) => {
                    error!("Could not poll controller for an assignment: {:?}", err);
                    error!("We'll try again in a minute");

                    delay_for(Duration::from_secs(60)).await;
                }
            }
        }
    }
}