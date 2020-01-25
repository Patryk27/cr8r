use anyhow::*;

use crate::{LxdConnector, LxdContainerName};

pub async fn config_set(
    conn: &LxdConnector,
    cname: &LxdContainerName,
    cfg_key: String,
    cfg_value: String,
) -> Result<()> {
    conn.invoke(&[
        "config".to_string(),
        "set".to_string(),
        cname.to_string(),
        cfg_key,
        cfg_value,
    ]).await?;

    Ok(())
}