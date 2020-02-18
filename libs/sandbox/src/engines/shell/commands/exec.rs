use anyhow::*;
use log::*;

use lib_process::Process;

use crate::engines::ShellSandboxEngine;

pub async fn exec(engine: &mut ShellSandboxEngine, cmd: &str) -> Result<()> {
    trace!("Executing: exec(cmd=`{}`)", cmd);

    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    // @todo `/bin/bash` shouldn't be hard-coded
    let status = Process::new("/bin/bash")
        .current_dir(&engine.config.root)
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