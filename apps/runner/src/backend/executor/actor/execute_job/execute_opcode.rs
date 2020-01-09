use anyhow::Result;

use lib_interop::domain::DJobOpcode;

use crate::backend::executor::ExecutorActor;

mod do_invoke_cmd;
mod do_log_custom_msg;
mod do_log_system_msg;
mod do_override_package;
mod do_override_toolchain;
mod do_patch_package;

impl ExecutorActor {
    pub(super) async fn execute_opcode(&mut self, opcode: DJobOpcode) -> Result<()> {
        use DJobOpcode::*;

        match opcode {
            LogSystemMsg { msg } => {
                self.do_log_system_msg(msg)
                    .await
            }

            LogCustomMsg { msg } => {
                self.do_log_custom_msg(msg)
                    .await
            }

            InvokeCmd { cmd } => {
                self.do_invoke_cmd(cmd)
                    .await
            }

            OverrideToolchain { project, version } => {
                self.do_override_toolchain(project, version)
                    .await
            }

            OverridePackage { project, name, version } => {
                self.do_override_package(project, name, version)
                    .await
            }

            PatchPackage { project, name, attachment_id } => {
                self.do_patch_package(project, name, attachment_id)
                    .await
            }
        }
    }
}