use anyhow::{anyhow, Result};
use lib_process::Process;

use crate::ShellEngine;

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
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Previous command returned a non-zero exit code"))
    }
}