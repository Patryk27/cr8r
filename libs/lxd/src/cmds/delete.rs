use crate::{cmds, LxdClient, LxdContainerName, Result};

pub async fn delete(lxd: &LxdClient, container: &LxdContainerName) -> Result<()> {
    cmds::invoke(lxd, &[
        "delete".to_string(),
        container.to_string(),
        "--force".to_string(),
    ]).await?;

    Ok(())
}