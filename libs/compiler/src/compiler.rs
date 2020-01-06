use std::collections::HashMap;

use lib_interop::domain::{DDefinition, DJob, DJobOpcode};

use crate::{CompilerBuilder, Environment, Project, ProjectName, Provider, ProviderName};

#[derive(Debug)]
pub struct Compiler {
    crate environment: Environment,
    crate providers: HashMap<ProviderName, Provider>,
    crate projects: HashMap<ProjectName, Project>,
}

impl Compiler {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::default()
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        let toolchain = definition.toolchain
            .as_ref()
            .map(|toolchain| &toolchain.version)
            .unwrap_or(&self.environment.default_toolchain);

        let mut jobs = Vec::new();

        for (project_name, project) in &self.projects {
            self.compile_project(toolchain, &mut jobs, project_name, project);
        }

        jobs
    }

    fn compile_project(&self, toolchain: &str, jobs: &mut Vec<DJob>, project_name: &ProjectName, project: &Project) {
        let mut opcodes = Vec::new();

        for provider_name in project.requirements() {
            let provider = &self.providers[provider_name];

            opcodes.push(DJobOpcode::log_system_msg(
                format!("Setting up requirement `{}`", provider_name)
            ));

            for command in provider.setup() {
                opcodes.push(DJobOpcode::exec(
                    command.inner()
                ));
            }
        }

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::exec(
            format!("git clone {} {}", project.repository(), project_name)
        ));

        // @todo apply package overrides

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Testing `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::exec(
            format!("cd {} && cargo test", project_name)
        ));

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Building `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::exec(
            format!("cd {} && cargo build", project_name)
        ));

        jobs.push(DJob {
            name: project_name.to_string(),
            toolchain: toolchain.to_owned(),
            opcodes,
        });
    }
}