use anyhow::Result;
use closure::*;

use lib_interop::domain::DEventType;
use lib_sandbox::SandboxListener;

use crate::backend::executor::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn init_sandbox(&mut self) -> Result<()> {
        self.journalist.dispatch(DEventType::SystemMsg {
            msg: "Initializing sandbox".to_string(),
        });

        let journalist = self.journalist.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone journalist, |cmd| {
                journalist.dispatch(DEventType::CustomMsg {
                    msg: format!("Executing: {}", cmd),
                });
            })),

            on_command_output: Some(box closure!(clone journalist, |msg| {
                journalist.dispatch(DEventType::ProcessMsg {
                    msg,
                });
            })),
        };

        self.sandbox
            .init(Some(listener))
            .await
    }
}