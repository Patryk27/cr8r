use log::*;
use tokio::time::{delay_for, Duration};

use lib_interop::domain::DExperimentId;
use lib_interop::proto::controller::p_prepare_assignment_reply::Assignment;
use lib_interop::proto::controller::PPrepareAssignmentReply;

use crate::system::Dispatcher;

impl Dispatcher {
    pub(super) async fn await_assignment(&mut self) -> DExperimentId {
        loop {
            debug!("Polling controller for a new assignment");

            let assignment = self.session
                .invoke(|client, runner_id| client.prepare_assignment(runner_id))
                .await;

            match assignment {
                Ok(PPrepareAssignmentReply { assignment: Some(Assignment::ExperimentId(experiment_id)) }) => {
                    info!("We've been assigned experiment [id={}]", experiment_id);
                    return experiment_id.into();
                }

                Ok(_) => {
                    delay_for(Duration::from_secs(1))
                        .await;
                }

                Err(err) => {
                    error!("Could not poll controller for an assignment: {:?}", err);
                    error!("We'll try again in a minute");

                    delay_for(Duration::from_secs(60))
                        .await;
                }
            }
        }
    }
}