use lib_lxd::LxdClient;
use lib_protocol::core::Assignment;

use crate::backend::{ExecutorRx, ExecutorStatus};
use crate::core::ExperimentClient;

pub struct ExecutorActor {
    rx: ExecutorRx,
    lxd: LxdClient,
    assignment: Assignment,
    client: ExperimentClient,
    container: String,
    status: ExecutorStatus,
}

// @todo this could be a regular function
macro_rules! await_lxd {
    ($self:expr, $lxd_result:expr) => {{
        let mut result = $lxd_result.unwrap();

        loop {
            use lib_lxd::LxdProcessMsg;
            use futures_util::StreamExt;

            match result.next().await {
                Some(LxdProcessMsg::Exited { status }) => {
                    break status.success();
                }

                Some(LxdProcessMsg::Stdout { line }) => {
                    $self.client
                        .report_process_stdout(line)
                        .await
                        .unwrap();
                }

                Some(LxdProcessMsg::Stderr { line }) => {
                    $self.client
                        .report_process_stderr(line)
                        .await
                        .unwrap();
                }

                None => {
                    break false;
                }
            }
        }
    }}
}

mod execute_scenario;
mod execute_step;
mod process_messages;
mod start;

impl ExecutorActor {
    pub fn new(rx: ExecutorRx, lxd: LxdClient, assignment: Assignment, client: ExperimentClient) -> Self {
        let container = format!("cr8r-{}", assignment.experiment_id);

        Self {
            rx,
            lxd,
            assignment,
            client,
            container,
            status: ExecutorStatus::Running,
        }
    }
}