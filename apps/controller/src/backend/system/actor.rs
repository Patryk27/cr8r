use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName, RunnerSecret};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Compiler, Experiment, Result, Runner, System, SystemCommand, SystemCommandRx};

use self::{
    experiments::*,
    runners::*,
};

mod experiments;
mod runners;

pub struct SystemActor {
    system: System,
    runner_secret: RunnerSecret,
    compiler: Compiler,
    runners: Runners,
    experiments: Experiments,
}

impl SystemActor {
    pub fn new(system: System, runner_secret: RunnerSecret, compiler: Compiler) -> Self {
        let runners = Runners::new(system.clone());
        let experiments = Experiments::new(system.clone());

        Self {
            system,
            runner_secret,
            compiler,
            runners,
            experiments,
        }
    }

    pub async fn start(mut self, mut rx: SystemCommandRx) {
        debug!("Actor started, entering event loop");

        while let Some(cmd) = rx.next().await {
            debug!("Processing command: {:?}", cmd);

            match cmd {
                SystemCommand::RequestAssignment { runner, tx } => {
                    let _ = tx.send(self.do_request_assignment(runner).await);
                }

                SystemCommand::FindExperiment { experiment, tx } => {
                    let _ = tx.send(
                        self.do_find_experiment(experiment),
                    );
                }

                SystemCommand::FindExperiments { tx } => {
                    let _ = tx.send(
                        self.do_find_experiments(),
                    );
                }

                SystemCommand::LaunchExperiment { experiment, tx } => {
                    let _ = tx.send(
                        self.do_launch_experiment(experiment),
                    );
                }

                SystemCommand::FindRunners { tx } => {
                    let _ = tx.send(
                        self.do_find_runners(),
                    );
                }

                SystemCommand::RegisterRunner { name, secret, tx } => {
                    let _ = tx.send(
                        self.do_register_runner(name, secret),
                    );
                }
            };
        }

        debug!("Actor orphaned, halting it");
    }

    async fn do_request_assignment(&mut self, runner: RunnerId) -> Result<Option<Assignment>> {
        if let Some(experiment) = self.experiments.take() {
            let assignment = experiment
                .start(runner)
                .await?;

            Ok(Some(assignment))
        } else {
            Ok(None)
        }
    }

    fn do_find_experiment(&self, experiment: ExperimentId) -> Result<Experiment> {
        self.experiments.get(&experiment)
    }

    fn do_find_experiments(&self) -> Vec<Experiment> {
        self.experiments.all()
    }

    fn do_launch_experiment(&mut self, definition: ExperimentDefinitionInner) -> Result<ExperimentId> {
        let scenarios = self.compiler.compile(&definition)?;
        let id = self.experiments.create(scenarios);

        Ok(id)
    }

    fn do_find_runners(&self) -> Vec<Runner> {
        self.runners.all()
    }

    fn do_register_runner(&mut self, name: RunnerName, secret: RunnerSecret) -> Result<RunnerId> {
        if secret != self.runner_secret {
            return Err("Invalid secret token".into());
        }

        self.runners.create(name)
    }
}