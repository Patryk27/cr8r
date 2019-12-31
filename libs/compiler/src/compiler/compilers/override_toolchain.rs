use lib_interop::contract::{CJob, CJobOpcode};

use crate::Compiler;

pub fn compile_override_toolchain(compiler: &Compiler, toolchain: &str) -> Vec<CJob> {
    let system = compiler.defaults.system.clone();
    let toolchain = toolchain.to_owned();

    let mut jobs = Vec::new();

    for (project_name, project) in &compiler.projects {
        let mut opcodes = Vec::new();

        for provider_name in project.requirements() {
            let provider = &compiler.providers[provider_name];

            opcodes.push(CJobOpcode::log_system_msg(
                format!("Setting up requirement: {}", provider_name)
            ));

            for command in provider.setup() {
                opcodes.push(CJobOpcode::exec(
                    command.inner()
                ));
            }
        }

        opcodes.push(CJobOpcode::log_system_msg(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(CJobOpcode::exec(
            format!("git clone {} {}", project.repository(), project_name)
        ));

        opcodes.push(CJobOpcode::log_system_msg(
            format!("Starting tests for `{}`", project_name)
        ));

        opcodes.push(CJobOpcode::exec(
            format!("cd {} && cargo test", project_name)
        ));

        opcodes.push(CJobOpcode::log_system_msg(
            format!("Starting build for `{}`", project_name)
        ));

        opcodes.push(CJobOpcode::exec(
            format!("cd {} && cargo build", project_name)
        ));

        jobs.push(CJob {
            name: project_name.to_string(),
            system: system.clone(),
            toolchain: toolchain.clone(),
            opcodes,
        });
    }

    jobs
}