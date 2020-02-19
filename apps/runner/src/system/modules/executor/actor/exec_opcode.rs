use anyhow::*;

use lib_core_actor::*;
use lib_interop::models::DJobOpcode;

use super::{ExecutorActor, ExecutorContext};

mod do_alter_dependency;
mod do_alter_toolchain;
mod do_emit;
mod do_execute;

impl ExecutorActor {
    pub(super) async fn exec_opcode(&mut self, context: &ExecutorContext, opcode: DJobOpcode) -> Result<ActorWorkflow> {
        use DJobOpcode::*;

        if self.handle_messages().actor_should_stop() {
            return Ok(ActorWorkflow::Stop);
        }

        match opcode {
            Emit { msg } => {
                self.do_emit(msg).await?
            }

            Execute { cmd } => {
                self.do_execute(cmd).await?
            }

            AlterToolchain { project, toolchain } => {
                self.do_alter_toolchain(project, toolchain).await?
            }

            AlterDependency { project, dependency } => {
                self.do_alter_dependency(context, project, dependency).await?
            }
        }

        Ok(ActorWorkflow::Continue)
    }
}