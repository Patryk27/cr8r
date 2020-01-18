use std::collections::HashMap;

use lib_interop::domain::{DDefinition, DJob, DJobOpcode};
use lib_interop::domain::definition_inner::DDependencyAction;
use lib_interop::domain::job::opcode::DOverrideDependencyAction;

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
            jobs.push(self.compile_project(
                definition,
                project_name,
                project,
            ));
        }

        jobs
    }

    fn compile_project(
        &self,
        definition: &DDefinition,
        project_name: &ProjectName,
        project_def: &ProjectDef,
    ) -> DJob {
        let mut opcodes = Vec::new();

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::invoke_cmd(
            format!("git clone {} {}", project_def.repository, project_name)
        ));

        let req_count = project_def.requirements.len();

        for (req_id, req_name) in project_def.requirements.iter().enumerate() {
            let req_provider = &self.providers[req_name];

            opcodes.push(DJobOpcode::log_system_msg(format!(
                "Setting up requirement {}/{} `{}`",
                req_id,
                req_count,
                req_name,
            )));

            for command in &req_provider.setup {
                opcodes.push(DJobOpcode::invoke_cmd(
                    command.inner()
                ));
            }
        }

        if definition.toolchain.is_some() || !definition.dependencies.is_empty() {
            opcodes.push(DJobOpcode::log_system_msg(
                "Preparing the environment",
            ));
        }

        let toolchain = if let Some(toolchain) = &definition.toolchain {
            &toolchain.version
        } else {
            &self.environment.default_toolchain
        };

        opcodes.push(DJobOpcode::override_toolchain(
            project_name,
            toolchain,
        ));

        for dependency in &definition.dependencies {
            let action = match &dependency.action {
                DDependencyAction::AttachmentOverride { attachment_id } => {
                    DOverrideDependencyAction::UseAttachment {
                        attachment_id: attachment_id.to_owned(),
                    }
                }

                DDependencyAction::VersionOverride { version } => {
                    DOverrideDependencyAction::UseVersion {
                        version: version.to_owned(),
                    }
                }
            };

            opcodes.push(DJobOpcode::override_dependency(
                project_name,
                &dependency.registry,
                &dependency.name,
                action,
            ));
        }

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Building `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::invoke_cmd(
            format!("cd {} && cargo build", project_name)
        ));

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Testing `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::invoke_cmd(
            format!("cd {} && cargo test", project_name)
        ));

        DJob {
            name: project_name.to_string(),
            opcodes,
        }
    }
}