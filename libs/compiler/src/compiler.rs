use std::collections::HashMap;

use lib_interop::domain::{DDefinition, DJob, DJobOpcode};
use lib_interop::domain::definition_inner::DPackage;

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

    fn compile_project(&self, definition: &DDefinition, project_name: &ProjectName, project: &ProjectDef) -> DJob {
        let mut opcodes = Vec::new();

        let req_count = project.requirements.len();

        for (req_id, req_name) in project.requirements.iter().enumerate() {
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

        opcodes.push(DJobOpcode::log_system_msg(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(DJobOpcode::invoke_cmd(
            format!("git clone {} {}", project.repository, project_name)
        ));

        for (package_name, package) in &definition.packages {
            match package {
                DPackage::Overridden { version } => {
                    opcodes.push(DJobOpcode::override_package(
                        project_name.to_owned(),
                        package_name,
                        version,
                    ));
                }

                DPackage::Patched { attachment_id } => {
                    opcodes.push(DJobOpcode::patch_package(
                        project_name.to_owned(),
                        project_name,
                        attachment_id.to_owned(),
                    ));
                }
            }
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