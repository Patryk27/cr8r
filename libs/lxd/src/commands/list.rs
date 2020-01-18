use anyhow::{Context, Result};

use crate::{commands, LxdClient, LxdContainer};

pub async fn list(lxd: &LxdClient) -> Result<Vec<LxdContainer>> {
    let output = commands::invoke(lxd, &[
        "list".to_string(),
        "--format=json".to_string(),
    ]).await?;

    let containers = serde_json::from_str(&output)
        .context("Could not parse response from LXD")?;

    Ok(containers)
}