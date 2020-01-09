use log::*;

use lib_interop::domain::{DAssignment, DEventType};
use lib_sandbox::Sandbox;

use crate::backend::{ExecutorStatus, Journalist};
use crate::backend::executor::ExecutorRx;

mod execute_job;
mod process_messages;

pub struct ExecutorActor {
    rx: ExecutorRx,
    pub(super) sandbox: Sandbox,
    pub(super) assignment: DAssignment,
    pub(super) journalist: Journalist,
    pub(super) status: ExecutorStatus,
}

impl ExecutorActor {
    pub fn new(
        rx: ExecutorRx,
        sandbox: Sandbox,
        assignment: DAssignment,
        journalist: Journalist,
    ) -> Self {
        Self {
            rx,
            sandbox,
            assignment,
            journalist,
            status: ExecutorStatus::Running,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        self.journalist.dispatch(DEventType::ExperimentStarted);

        self.process_messages_and_yield()
            .await;

        let jobs = self.assignment.jobs
            .drain(..)
            .collect(): Vec<_>;

        for (id, job) in jobs.into_iter().enumerate() {
            self.journalist.dispatch(DEventType::JobStarted { id });

            let result = self
                .execute_job(job)
                .await
                .map_err(|err| err.to_string());

            self.journalist.dispatch(DEventType::JobCompleted { id, result });
        }

        self.journalist.dispatch(DEventType::ExperimentCompleted);

        self.status = ExecutorStatus::Completed;

        debug!("Actor finished working, entering event loop");

        self.process_messages_and_wait()
            .await;

        debug!("Actor orphaned, halting");
    }
}