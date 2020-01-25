use anyhow::*;
use log::*;

use crate::engines::LxdSandboxEngine;

pub async fn exec(engine: &mut LxdSandboxEngine, cmd: &str) -> Result<()> {
    trace!("Executing: exec(cmd=`{}`)", cmd);

    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    engine.client
        .exec(&engine.config.container, &["bash", "-c", cmd])
        .await?;

    Ok(())
}