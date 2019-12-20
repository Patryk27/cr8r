use tokio::process::Command;
use tokio::stream::StreamExt;

use lib_process::{ProcessEvent, spawn};

use crate::{Error, Result, ShellEngine};

pub async fn exec(engine: &mut ShellEngine, cmd: &str) -> Result<()> {
    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    // @todo this is cheesy
    let mut command = Command::new("/usr/bin/bash");
    command.current_dir(&engine.dir);
    command.args(&["-c", cmd]);

    let mut events = spawn(command);

    while let Some(event) = events.next().await {
        match event {
            ProcessEvent::Crashed { .. } => {
                // @todo do something with `err`

                return Err(Error::CommandFailed);
            }

            ProcessEvent::Exited { status } => {
                return if status.success() {
                    Ok(())
                } else {
                    Err(Error::CommandFailed)
                };
            }

            ProcessEvent::Printed { line } => {
                if let Some(handler) = &engine.listener.on_command_output {
                    handler(line);
                }
            }
        }
    }

    // @todo command terminated abruptly
    Err(Error::CommandFailed)
}