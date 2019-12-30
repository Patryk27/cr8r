use log::*;

use lib_interop::contract::{CAssignment, CEventType};
use lib_sandbox::Sandbox;

use crate::backend::{ExecutorStatus, Journalist};
use crate::backend::executor::ExecutorRx;

mod perform_job;
mod process_messages;

pub struct ExecutorActor {
    rx: ExecutorRx,
    pub(super) sandbox: Sandbox,
    pub(super) assignment: CAssignment,
    pub(super) journalist: Journalist,
    pub(super) status: ExecutorStatus,
}

impl ExecutorActor {
    pub fn new(
        rx: ExecutorRx,
        sandbox: Sandbox,
        assignment: CAssignment,
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

        self.journalist.dispatch(CEventType::ExperimentStarted);

        self.process_messages_and_yield()
            .await;

        let jobs = self.assignment.jobs
            .drain(..)
            .collect(): Vec<_>;

        for (id, job) in jobs.into_iter().enumerate() {
            self.journalist.dispatch(CEventType::JobStarted { id });

            let result = self
                .perform_job(job)
                .await;

            self.journalist.dispatch(CEventType::JobCompleted { id, result });
        }

        self.journalist.dispatch(CEventType::ExperimentCompleted);

        self.status = ExecutorStatus::Completed;

        debug!("Actor finished working, entering event loop");

        self.process_messages_and_wait()
            .await;

        debug!("Actor orphaned, halting");
    }
}