use crate::{commands, LxdClient, LxdContainerName, Result};

pub async fn delete(lxd: &LxdClient, container: &LxdContainerName) -> Result<()> {
    commands::invoke(lxd, &[
        "delete".to_string(),
        container.to_string(),
        "--force".to_string(),
    ]).await?;

    Ok(())
}