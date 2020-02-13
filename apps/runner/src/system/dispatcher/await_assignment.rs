use std::convert::TryInto;

use log::*;
use tokio::time::{delay_for, Duration};

use lib_interop::domain::DAssignment;
use lib_interop::proto::controller::PGetAssignmentReply;

use crate::system::Dispatcher;

impl Dispatcher {
    pub(super) async fn await_assignment(&mut self) -> DAssignment {
        loop {
            debug!("Polling controller for a new assignment");

            let assignment = self.session
                .invoke(|client, runner_id| client.get_assignment(runner_id))
                .await;

            match assignment {
                Ok(PGetAssignmentReply { assignment: Some(assignment) }) => {
                    let assignment = assignment
                        .try_into()
                        .unwrap(): DAssignment; // @todo

                    info!("We've been assigned experiment [id={}]", assignment.experiment.id);

                    return assignment;
                }

                Ok(PGetAssignmentReply { assignment: None }) => {
                    debug!("Got nothing");
                    debug!("We'll try again in a few seconds");

                    delay_for(Duration::from_secs(2))
                        .await;
                }

                Err(err) => {
                    error!("Couldn't poll controller for an assignment: {:?}", err);
                    error!("We'll try again in a minute");

                    delay_for(Duration::from_secs(60))
                        .await;
                }
            }
        }
    }
}