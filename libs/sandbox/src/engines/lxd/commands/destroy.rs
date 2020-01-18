use anyhow::Result;
use log::*;

use crate::engines::LxdEngine;

pub async fn destroy(engine: &mut LxdEngine) -> Result<()> {
    debug!("destroy");

    engine.client
        .delete(&engine.container)
        .await?;

    Ok(())
}
