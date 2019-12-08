use lib_protocol::core::{Scenario, scenario_step, ScenarioStep};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::Result;
use crate::core::Ecosystem;

pub struct Compiler {
    ecosystem: Ecosystem,
}

macro_rules! add_step {
    ($steps:expr, exec($command:expr)) => {
        $steps.push(ScenarioStep {
            op: Some(scenario_step::Op::Exec(scenario_step::Exec {
                command: $command.into(),
            })),
        });
    };

    ($steps:expr, print($message:expr)) => {
        $steps.push(ScenarioStep {
            op: Some(scenario_step::Op::Print(scenario_step::Print {
                message: $message.into(),
            })),
        });
    };
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Self {
        Self { ecosystem }
    }

    pub fn compile(&self, definition: &ExperimentDefinitionInner) -> Result<Vec<Scenario>> {
        let mut system = self.ecosystem.environment.default_system.clone();
        let mut toolchain = self.ecosystem.environment.default_toolchain.clone();

        match definition {
            ExperimentDefinitionInner::TrySystem(experiment) => {
                system = experiment.system.clone();
            }

            ExperimentDefinitionInner::TryToolchain(experiment) => {
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
                    add_step! { steps, exec(cmd) }
                }
            }

            add_step! { steps, print("Cloning project") }
            add_step! { steps, exec(format!("git clone {} project && cd project", project.repository)) }

            add_step! { steps, print("Starting tests") }
            add_step! { steps, exec("cargo test") }

            add_step! { steps, print("Starting build") }
            add_step! { steps, exec("cargo build") }

            scenarios.push(Scenario {
                project: project_name.clone(),
                system: system.clone(),
                toolchain: toolchain.clone(),
                steps,
            });
        }

        Ok(scenarios)
    }
}