use futures_util::StreamExt;

use lib_process::{ProcessEvent, spawn};

use crate::{Error, LxdClient, Result};

pub async fn invoke(lxd: &LxdClient, args: &[String]) -> Result<String> {
    let mut rx = spawn(&lxd.path, args);
    let mut output = String::new();

    while let Some(event) = rx.next().await {
        match event {
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