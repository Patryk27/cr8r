use snafu::ResultExt;

use lib_process::Process;

use crate::{Error, Result, ShellEngine};
use crate::engines::shell::error;

pub async fn exec(engine: &mut ShellEngine, cmd: &str) -> Result<()> {
    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    let status = Process::new("/usr/bin/bash")
        .current_dir(&engine.root)
        .args(&["-c", cmd])
        .listener(box |line| {
            if let Some(handler) = &engine.listener.on_command_output {
                handler(line);
            }
        })
        .spawn()
        .await
        .context(error::CommandNotStarted)?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed)
    }
}