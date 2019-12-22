use snafu::ResultExt;

use crate::{cmds, error, LxdClient, LxdContainer, Result};

pub async fn list(lxd: &LxdClient) -> Result<Vec<LxdContainer>> {
    let output = cmds::invoke(lxd, &[
        "list".to_string(),
        "--format=json".to_string(),
    ]).await?;

    let containers = serde_json::from_str(&output)
        .context(error::ClientReturnedGarbage)?;

    Ok(containers)
}