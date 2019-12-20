use tokio::process::Command;
use tokio::stream::StreamExt;

use lib_process::{ProcessEvent, spawn};

use crate::{Error, LxdClient, Result};

pub async fn invoke(lxd: &LxdClient, args: &[String]) -> Result<String> {
    let mut cmd = Command::new(&lxd.path);
    cmd.args(args);

    let mut events = spawn(cmd);
    let mut output = String::new();

    while let Some(event) = events.next().await {
        match event {
            ProcessEvent::Crashed { .. } => {
                // @todo do something with `err`

                return Err(Error::CommandFailed);
            }

            ProcessEvent::Exited { status } => {
                return if status.success() {
                    Ok(output)
                } else {
                    Err(Error::CommandFailed)
                };
            }

            ProcessEvent::Printed { line } => {
                output.push_str(&line);
                output.push('\n');

                if let Some(handler) = &lxd.listener.on_output {
                    handler(line);
                }
            }
        }
    }

    Err(Error::CommandTerminatedAbruptly)
}