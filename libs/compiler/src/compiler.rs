use std::collections::HashMap;

use lib_interop::models::{DDefinition, DJob, DJobOpcode};
use lib_interop::models::definition::DToolchainDef;

use crate::{Environment, ProjectDef, ProjectName, ProviderDef, ProviderName};

pub use self::builder::*;

mod builder;

#[derive(Debug, Eq, PartialEq)]
pub struct Compiler {
    crate environment: Environment,
    crate providers: HashMap<ProviderName, ProviderDef>,
    crate projects: HashMap<ProjectName, ProjectDef>,
}

impl Compiler {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::default()
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        let mut jobs = Vec::new();

        for (project_name, project) in &self.projects {
            let job_id = 1 + jobs.len() as u32;

            jobs.push(self.compile_project(
                job_id,
                definition,
                project_name,
                project,
            ));
        }

        jobs
    }

    fn compile_project(
        &self,
        job_id: u32,
        definition: &DDefinition,
        project_name: &ProjectName,
        project_def: &ProjectDef,
    ) -> DJob {
        let mut opcodes = Vec::new();

        opcodes.push(DJobOpcode::emit(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::execute(
            format!("git clone {} {}", project_def.repository, project_name)
        ));

        let req_count = project_def.requirements.len();

        for (req_id, req_name) in project_def.requirements.iter().enumerate() {
            let req_provider = &self.providers[req_name];

            opcodes.push(DJobOpcode::emit(format!(
                "Setting up requirement {}/{} `{}`",
                req_id,
                req_count,
                req_name,
            )));

            for command in &req_provider.setup {
                opcodes.push(DJobOpcode::execute(
                    command.inner()
                ));
            }
        }

        if definition.toolchain.is_some() || !definition.dependencies.is_empty() {
            opcodes.push(DJobOpcode::emit(
                "Preparing the environment",
            ));
        }

        let toolchain = if let Some(toolchain) = &definition.toolchain {
            &toolchain.toolchain
        } else {
            &self.environment.default_toolchain
        };

        opcodes.push(DJobOpcode::alter_toolchain(
            project_name,
            DToolchainDef {
                toolchain: toolchain.to_string(),
            },
        ));

        for dependency in &definition.dependencies {
            opcodes.push(DJobOpcode::patch_dependency(
                project_name,
                dependency.to_owned(),
            ));
        }

        opcodes.push(DJobOpcode::emit(
            format!("Building `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::execute(
            format!("cd {} && cargo build", project_name)
        ));

        opcodes.push(DJobOpcode::emit(
            format!("Testing `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::execute(
            format!("cd {} && cargo test", project_name)
        ));

        DJob {
            id: job_id.into(),
            name: project_name.into(),
            opcodes,
        }
    }
}