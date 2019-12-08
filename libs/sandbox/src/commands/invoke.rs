use futures_util::StreamExt;

use lib_lxd::{LxdClient, LxdEvent, LxdEventRx, Result as LxdResult};

use crate::{Error, Result, Sandbox};

impl Sandbox {
    crate async fn invoke(
        &self,
        action: impl FnOnce(&LxdClient) -> LxdResult<LxdEventRx>,
    ) -> Result<()> {
        let mut rx = action(&self.lxd)?;

        while let Some(event) = rx.next().await {
            match event {
                LxdEvent::Exited { status } => {
                    return if status.success() {
                        Ok(())
                    } else {
                        Err(Error::CommandFailed)
                    };
                }

                LxdEvent::Stdout { line } => {
                    if let Some(notify) = &self.listener.on_command_stdout {
                        notify(line);
                    }
                }

                LxdEvent::Stderr { line } => {
                    if let Some(notify) = &self.listener.on_command_stderr {
                        notify(line);
                    }
                }
            }
        }

        Err(Error::CommandFailed)
    }
}