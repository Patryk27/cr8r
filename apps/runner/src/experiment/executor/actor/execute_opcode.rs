use anyhow::Result;

use lib_actor::ActorWorkflow;
use lib_interop::domain::DJobOpcode;

use super::super::ExperimentExecutorActor;

mod do_invoke_cmd;
mod do_log_custom_msg;
mod do_log_system_msg;
mod do_override_dependency;
mod do_override_toolchain;
mod do_patch_dependency;

impl ExperimentExecutorActor {
    pub(super) async fn execute_opcode(&mut self, opcode: DJobOpcode) -> Result<ActorWorkflow> {
        if self.handle_messages().should_stop() {
            return Ok(ActorWorkflow::Stop);
        }

        use DJobOpcode::*;

        match opcode {
            LogSystemMsg { msg } => {
                self.do_log_system_msg(msg)
                    .await?
            }

            LogCustomMsg { msg } => {
                self.do_log_custom_msg(msg)
                    .await?
            }

            InvokeCmd { cmd } => {
                self.do_invoke_cmd(cmd)
                    .await?
            }

            OverrideToolchain { project, tc_version } => {
                self.do_override_toolchain(project, tc_version)
                    .await?
            }

            OverrideDependency { project, dep_registry, dep_name, dep_version } => {
                self.do_override_dependency(project, dep_registry, dep_name, dep_version)
                    .await?
            }

            PatchDependency { project, dep_registry, dep_name, dep_source_attachment_id } => {
                self.do_patch_dependency(project, dep_registry, dep_name, dep_source_attachment_id)
                    .await?
            }
        }

        Ok(ActorWorkflow::Continue)
    }
}