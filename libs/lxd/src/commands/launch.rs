use crate::{commands, LxdClient, LxdContainerName, LxdImageName, Result};

pub async fn launch(lxd: &LxdClient, image: &LxdImageName, container: &LxdContainerName) -> Result<()> {
    commands::invoke(lxd, &[
        "launch".to_string(),
        image.to_string(),
        container.to_string(),
        "-csecurity.nesting=true".to_string(),
        "-csecurity.privileged=true".to_string(),
    ]).await?;

    Ok(())
}