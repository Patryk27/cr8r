use anyhow::*;

use lib_core_actor::*;
use lib_interop::domain::DJobOpcode;

use super::super::ExperimentExecutorActor;

mod do_alter_dependency;
mod do_execute;
mod do_log_custom_msg;
mod do_log_system_msg;
mod do_alter_toolchain;

impl ExperimentExecutorActor {
    pub(super) async fn execute_opcode(&mut self, opcode: DJobOpcode) -> Result<ActorWorkflow> {
        if self.handle_messages().actor_should_stop() {
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

            Execute { cmd } => {
                self.do_execute(cmd)
                    .await?
            }

            AlterToolchain { project, toolchain } => {
                self.do_alter_toolchain(project, toolchain)
                    .await?
            }

            AlterDependency { project, dependency } => {
                self.do_alter_dependency(project, dependency)
                    .await?
            }
        }

        Ok(ActorWorkflow::Continue)
    }
}