use lib_protocol::core::p_experiment_definition::Op as PExperimentDefinitionOp;
use lib_protocol::core::p_scenario_step::{self, Op as PScenarioStepOp};
use lib_protocol::core::PScenario;
use lib_protocol::core::PScenarioStep;

use crate::backend::Result;
use crate::core::Ecosystem;

pub struct Compiler {
    ecosystem: Ecosystem,
}

macro_rules! add_step {
    ($steps:expr, run($command:expr)) => {
        $steps.push(PScenarioStep {
            op: Some(PScenarioStepOp::Run(p_scenario_step::PRun {
                command: $command.into(),
            })),
        });
    };

    ($steps:expr, print($message:expr)) => {
        $steps.push(PScenarioStep {
            op: Some(PScenarioStepOp::Print(p_scenario_step::PPrint {
                message: $message.into(),
            })),
        });
    };
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Self {
        Self { ecosystem }
    }

    pub fn compile(&self, definition: &PExperimentDefinitionOp) -> Result<Vec<PScenario>> {
        let mut system = self.ecosystem.environment.default_system.clone();
        let mut toolchain = self.ecosystem.environment.default_toolchain.clone();

        match definition {
            PExperimentDefinitionOp::TrySystem(experiment) => {
                system = experiment.system.clone();
            }

            PExperimentDefinitionOp::TryToolchain(experiment) => {
                toolchain = experiment.toolchain.clone();
            }
        }

        let mut scenarios = Vec::with_capacity(
            self.ecosystem.fauna.len(),
        );

        for (project_name, project) in &self.ecosystem.fauna {
            let mut steps = Vec::new();

            for requirement in &project.requirements {
                let provider = &self.ecosystem.flora[requirement];

                add_step! { steps, print(format!("Setting up requirement: {}", requirement)) }

                for cmd in &provider.setup {
                    add_step! { steps, run(cmd) }
                }
            }

            add_step! { steps, print("Cloning project") }
            add_step! { steps, run(format!("git clone {} project && cd project", project.repository)) }

            add_step! { steps, print("Starting tests") }
            add_step! { steps, run("cargo test") }

            add_step! { steps, print("Starting build") }
            add_step! { steps, run("cargo build") }

            scenarios.push(PScenario {
                project: project_name.clone(),
                system: system.clone(),
                toolchain: toolchain.clone(),
                steps,
            });
        }

        Ok(scenarios)
    }
}