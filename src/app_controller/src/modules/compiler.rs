use lib_protocol::{ExecutionPlan, ExecutionStep, ExperimentDefinition};

use crate::config::Ecosystem;

pub struct Compiler {
    ecosystem: Ecosystem,
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Self {
        Self { ecosystem }
    }

    pub fn compile(&self, definition: &ExperimentDefinition) -> Vec<ExecutionPlan> {
        let mut os = self.ecosystem.environment.default_os.clone();
        let mut toolchain = self.ecosystem.environment.default_toolchain.clone();

        let mut plans = Vec::with_capacity(
            self.ecosystem.fauna.len(),
        );

        match definition {
            ExperimentDefinition::TryOs { os: new_os } => {
                os = new_os.clone();
            }

            ExperimentDefinition::TryToolchain { toolchain: new_toolchain } => {
                toolchain = new_toolchain.clone();
            }
        }

        for (project_name, project) in &self.ecosystem.fauna {
            let mut steps = Vec::new();

            steps.push(ExecutionStep::Log {
                message: "Setting up requirements.".into(),
            });

            for requirement_name in &project.requires {
                let provider = &self.ecosystem.flora[requirement_name];

                steps.push(ExecutionStep::Log {
                    message: format!("Setting up requirement: {}", requirement_name),
                });

                steps.push(ExecutionStep::RunCommands {
                    commands: provider.setup
                        .iter()
                        .cloned()
                        .collect()
                });
            }

            steps.push(ExecutionStep::Log {
                message: "Starting the experiment.".into(),
            });

            steps.push(ExecutionStep::Start);

            plans.push(ExecutionPlan {
                project_name: project_name.clone(),
                project_repository: project.repository.clone(),
                os: os.clone(),
                toolchain: toolchain.clone(),
                steps,
            });
        }

        plans
    }
}