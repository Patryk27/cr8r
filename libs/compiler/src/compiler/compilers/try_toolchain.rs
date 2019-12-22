use lib_interop::contract::{CProgram, CProgramOpcode};

use crate::Compiler;

pub fn compile_try_toolchain(compiler: &Compiler, toolchain: &str) -> CProgram {
    let system = compiler.defaults.system.clone();
    let toolchain = toolchain.to_owned();
    let mut opcodes = Vec::new();

    for (project_name, project) in &compiler.projects {
        for provider_name in project.requirements() {
            let provider = &compiler.providers[provider_name];

            opcodes.push(CProgramOpcode::log_system_msg(
                format!("Setting up requirement: {}", provider_name)
            ));

            for command in provider.setup() {
                opcodes.push(CProgramOpcode::exec(
                    command.inner()
                ));
            }
        }

        opcodes.push(CProgramOpcode::log_system_msg(
            format!("Cloning `{}`", project_name)
        ));

        opcodes.push(CProgramOpcode::exec(
            format!("git clone {} {}", project.repository(), project_name)
        ));

        opcodes.push(CProgramOpcode::log_system_msg(
            format!("Starting tests for `{}`", project_name)
        ));

        opcodes.push(CProgramOpcode::exec(
            format!("cd {} && cargo test", project_name)
        ));

        opcodes.push(CProgramOpcode::log_system_msg(
            format!("Starting build for `{}`", project_name)
        ));

        opcodes.push(CProgramOpcode::exec(
            format!("cd {} && cargo build", project_name)
        ));
    }

    CProgram { system, toolchain, opcodes }
}
