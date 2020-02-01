use anyhow::*;

use crate::{LxdConnector, LxdContainer};

pub async fn list(conn: &LxdConnector) -> Result<Vec<LxdContainer>> {
    let containers = conn.invoke_silent(&[
        "list".to_string(),
        "--format=json".to_string(),
    ]).await?;

    let containers = serde_json::from_str(&containers)
        .context("Could not understand response from LXD")?;

    Ok(containers)
}