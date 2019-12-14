use lib_sandbox_lxd::{LxdClient, LxdResponseEvent, LxdResponseStream, Result as LxdResult};

use crate::{Error, Result, Sandbox};

impl Sandbox {
    crate async fn invoke(
        &self,
        action: impl FnOnce(&LxdClient) -> LxdResult<LxdResponseStream>,
    ) -> Result<()> {
        let mut response = action(&self.lxd)?;

        while let Some(event) = response.next().await {
            match event {
                LxdResponseEvent::Exited { status } => {
                    return if status.success() {
                        Ok(())
                    } else {
                        Err(Error::CommandFailed)
                    };
                }

                LxdResponseEvent::Printed { line } => {
                    if let Some(notify) = &self.listener.on_command_printed {
                        notify(line);
                    }
                }
            }
        }

        Err(Error::CommandFailed)
    }
}