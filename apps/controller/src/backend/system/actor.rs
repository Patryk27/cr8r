use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Compiler, Experiment, Result, Runner, System, SystemMsg, SystemRx};

use self::{
    experiments::*,
    runners::*,
};

mod experiments;
mod runners;

pub struct SystemActor {
    rx: SystemRx,
    compiler: Compiler,
    runners: Runners,
    experiments: Experiments,
}

impl SystemActor {
    pub fn new(
        rx: SystemRx,
        system: System,
        compiler: Compiler,
    ) -> Self {
        let runners = Runners::new(system.clone());
        let experiments = Experiments::new(system.clone());

        Self {
            rx,
            compiler,
            runners,
            experiments,
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started, entering event loop");

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            match msg {
                SystemMsg::RequestAssignment { runner, tx } => {
                    let _ = tx.send(self.request_assignment(runner).await);
                }

                SystemMsg::FindExperiment { experiment, tx } => {
                    let _ = tx.send(self.find_experiment(experiment));
                }

                SystemMsg::FindExperiments { tx } => {
                    let _ = tx.send(self.find_experiments());
                }

                SystemMsg::LaunchExperiment { experiment, tx } => {
                    let _ = tx.send(self.launch_experiment(experiment));
                }

                SystemMsg::FindRunners { tx } => {
                    let _ = tx.send(self.find_runners());
                }

                SystemMsg::RegisterRunner { name, tx } => {
                    let _ = tx.send(self.register_runner(name));
                }
            };
        }

        debug!("Actor orphaned, halting");
    }

    async fn request_assignment(&mut self, runner: RunnerId) -> Result<Option<Assignment>> {
        if let Some(experiment) = self.experiments.take() {
            let assignment = experiment
                .start(runner)
                .await?;

            Ok(Some(assignment))
        } else {
            Ok(None)
        }
    }

    fn find_experiment(&self, experiment: ExperimentId) -> Result<Experiment> {
        self.experiments.get(&experiment)
    }

    fn find_experiments(&self) -> Vec<Experiment> {
        self.experiments.all()
    }

    fn launch_experiment(&mut self, definition: ExperimentDefinitionInner) -> Result<ExperimentId> {
        let scenarios = self.compiler.compile(&definition)?;
        let id = self.experiments.create(scenarios);

        Ok(id)
    }

    fn find_runners(&self) -> Vec<Runner> {
        self.runners.all()
    }

    fn register_runner(&mut self, name: RunnerName) -> Result<RunnerId> {
        self.runners.create(name)
    }
}