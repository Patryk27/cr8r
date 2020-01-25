use anyhow::*;
use log::*;

use crate::engines::LxdSandboxEngine;

pub async fn destroy(engine: &mut LxdSandboxEngine) -> Result<()> {
    debug!("Executing: destroy()");

    engine.client
        .delete(&engine.config.container)
        .await?;

    Ok(())
}
