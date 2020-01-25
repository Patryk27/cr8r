use anyhow::*;

use crate::{LxdConnector, LxdContainerName, LxdImageName};

pub async fn launch(
    conn: &LxdConnector,
    cimage: &LxdImageName,
    cname: &LxdContainerName,
) -> Result<()> {
    conn.invoke(&[
        "launch".to_string(),
        cimage.to_string(),
        cname.to_string(),
        "--ephemeral".to_string(),
        "-csecurity.nesting=true".to_string(),
        "-csecurity.privileged=true".to_string(),
    ]).await?;

    Ok(())
}