use anyhow::Result;
use log::*;

use crate::engines::LxdEngine;

pub async fn exec(engine: &mut LxdEngine, cmd: &str) -> Result<()> {
    debug!("exec :: cmd={}", cmd);

    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    engine.client
        .exec(&engine.container, &["bash", "-c", cmd])
        .await?;

    Ok(())
}