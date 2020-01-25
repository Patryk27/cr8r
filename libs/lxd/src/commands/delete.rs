use anyhow::*;

use crate::{LxdConnector, LxdContainerName};

pub async fn delete(conn: &LxdConnector, cname: &LxdContainerName) -> Result<()> {
    conn.invoke(&[
        "delete".to_string(),
        cname.to_string(),
        "--force".to_string(),
    ]).await?;

    Ok(())
}