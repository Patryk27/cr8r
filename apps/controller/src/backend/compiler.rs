use lib_protocol::core::p_experiment_def::Op as PExperimentDefOp;
use lib_protocol::core::p_experiment_step::{self, Op as PExperimentStepOp};
use lib_protocol::core::PExperimentStep;

use crate::backend::Result;
use crate::core::Ecosystem;

pub struct Compiler {
    ecosystem: Ecosystem,
}

macro_rules! add_step {
    ($steps:expr, exec($command:expr)) => {
        $steps.push(PExperimentStep {
            op: Some(PExperimentStepOp::Exec(p_experiment_step::PExec {
                cmd: $command.into(),
            })),
        });
    };

    ($steps:expr, log_system_msg($message:expr)) => {
        $steps.push(PExperimentStep {
            op: Some(PExperimentStepOp::LogSystemMsg(p_experiment_step::PLogSystemMsg {
                msg: $message.into(),
            })),
        });
    };
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Self {
        Self { ecosystem }
    }

    pub fn compile(&self, experiment_def: &PExperimentDefOp) -> Result<(String, String, Vec<PExperimentStep>)> {
        let system = self.ecosystem.environment.default_system.clone();
        let mut toolchain = self.ecosystem.environment.default_toolchain.clone();

        match experiment_def {
            PExperimentDefOp::TryToolchain(experiment) => {
                toolchain = experiment.toolchain.clone();
            }
        }

        let mut steps = Vec::new();

        for (project_name, project) in &self.ecosystem.fauna {
            for requirement in &project.requirements {
                let provider = &self.ecosystem.flora[requirement];

                add_step! { steps, log_system_msg(format!("Setting up requirement: {}", requirement)) }

                for cmd in &provider.setup {
                    add_step! { steps, exec(cmd) }
                }
            }

            add_step! { steps, log_system_msg(format!("Cloning `{}`", project_name)) }
            add_step! { steps, exec(format!("git clone {} project", project.repository)) }

            add_step! { steps, log_system_msg(format!("Starting tests for `{}`", project_name)) }
            add_step! { steps, exec("cd project && cargo test") }

            add_step! { steps, log_system_msg(format!("Starting build for `{}`", project_name)) }
            add_step! { steps, exec("cd project && cargo build") }
        }

        Ok((system, toolchain, steps))
    }
}