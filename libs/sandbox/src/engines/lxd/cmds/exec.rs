use crate::{LxdEngine, Result};

pub async fn exec(engine: &mut LxdEngine, cmd: &str) -> Result<()> {
    if let Some(handler) = &engine.listener.on_command_executed {
        handler(cmd.to_string());
    }

    engine.lxd
        .exec(&engine.container, &["bash", "-c", cmd])
        .await?;

    Ok(())
}