use std::result;
use std::sync::Arc;

use bastion::prelude::{BastionContext, ChildRef};

use lib_protocol::core::Assignment;
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::system::{Compiler, ExperimentId, ExperimentSession, Result, RunnerName, RunnerSecret, RunnerSession};

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone)]
pub struct System {
    conn: ChildRef,
}

macro_rules! ask {
    ($self:expr, $cmd:expr) => {
        $self.conn
            .ask($cmd)
            .unwrap()
            .await
            .unwrap()
            .downcast()
            .unwrap()
    }
}

impl System {
    pub async fn start(
        runner_secret: Arc<RunnerSecret>,
        compiler: Arc<Compiler>,
        ctx: BastionContext,
    ) -> result::Result<(), ()> {
        Actor::new(runner_secret, compiler)
            .start(ctx)
            .await
    }

    pub async fn abort_experiment(&self, experiment: ExperimentId) -> Result<()> {
        ask!(self, Command::AbortExperiment { experiment })
    }

    pub async fn launch_experiment(&self, experiment: ExperimentDefinitionInner) -> Result<(ExperimentSession, u32)> {
        ask!(self, Command::LaunchExperiment { experiment })
    }

    pub async fn report_experiment(&self, runner: RunnerToken, experiment: ExperimentId, report: ()) -> Result<()> {
        ask!(self, Command::ReportExperiment { runner, experiment, report })
    }

    pub async fn request_experiment(&self, runner: RunnerToken) -> Result<Option<Assignment>> {
        ask!(self, Command::RequestExperiment { runner })
    }

    pub async fn register_runner(&self, name: RunnerName, secret: RunnerSecret) -> Result<RunnerSession> {
        ask!(self, Command::RegisterRunner { name, secret })
    }
}

impl Into<System> for ChildRef {
    fn into(self) -> System {
        System { conn: self }
    }
}