use std::result;
use std::sync::Arc;

use bastion::context::BastionContext;
use log::*;

use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::system::{Command, Compiler, ExperimentId, Result, RunnerId, RunnerName, RunnerSecret};

use self::{
    experiments::*,
    runners::*,
};

mod experiments;
mod runners;

pub struct Actor {
    runner_secret: Arc<RunnerSecret>,
    compiler: Arc<Compiler>,
    runners: Runners,
    experiments: Experiments,
}

impl Actor {
    pub fn new(runner_secret: Arc<RunnerSecret>, compiler: Arc<Compiler>) -> Self {
        Self {
            runner_secret,
            compiler,
            runners: Runners::new(),
            experiments: Experiments::new(),
        }
    }

    pub async fn start(mut self, ctx: BastionContext) -> result::Result<(), ()> {
        debug!("System actor started, entering the event loop.");

        loop {
            let mut packet = ctx.recv().await?;

            let sender = packet.take_sender().unwrap();
            let command = packet.downcast().unwrap(): Command;

            debug!("Processing command: {:?}", command);

            match command {
                Command::AbortExperiment { experiment } => {
                    unimplemented!()
                }

                Command::LaunchExperiment { experiment } => {
                    let _ = sender.send(self.launch_experiment(experiment).await);
                }

                Command::ReportExperiment { runner, experiment, report } => {
                    unimplemented!()
                }

                Command::RequestExperiment { runner } => {
                    let _ = sender.send(self.request_experiment(runner).await);
                }

                Command::RegisterRunner { name, secret } => {
                    let _ = sender.send(self.register_runner(name, secret).await);
                }
            };
        }
    }

    async fn launch_experiment(&mut self, definition: ExperimentDefinitionInner) -> Result<(ExperimentId, u32)> {
        let scenarios = self.compiler.compile(&definition);
        let id = self.experiments.create(definition, scenarios);

        Ok((id, 1)) // @todo return proper position in queue
    }

    async fn request_experiment(&mut self, runner: RunnerId) -> Result<Option<ExperimentId>> {
        if let Some(experiment) = self.experiments.take_pending() {
            self.experiments.claim(experiment, runner);
            Ok(Some(experiment))
        } else {
            Ok(None)
        }
    }

    async fn register_runner(&mut self, name: RunnerName, secret: RunnerSecret) -> Result<RunnerId> {
        if secret != *self.runner_secret {
            return Err("Invalid secret token".to_string());
        }

        self.runners.register(name)
    }
}