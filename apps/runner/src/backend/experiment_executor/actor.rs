use lib_protocol::core::Assignment;
use lib_sandbox::Sandbox;

use crate::backend::{ExecutorStatus, ExperimentExecutorRx, ExperimentReporter};

mod execute_scenario;
mod execute_step;
mod process_messages;
mod start;

pub struct ExperimentExecutorActor {
    rx: ExperimentExecutorRx,
    sandbox: Sandbox,
    assignment: Assignment,
    reporter: ExperimentReporter,
    status: ExecutorStatus,
}

impl ExperimentExecutorActor {
    pub fn new(
        rx: ExperimentExecutorRx,
        sandbox: Sandbox,
        assignment: Assignment,
        reporter: ExperimentReporter,
    ) -> Self {
        Self {
            rx,
            sandbox,
            assignment,
            reporter,
            status: ExecutorStatus::Running,
        }
    }
}