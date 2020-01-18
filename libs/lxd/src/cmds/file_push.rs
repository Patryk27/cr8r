use std::path::Path;

use anyhow::Result;

use crate::{cmds, LxdClient, LxdContainerName};

pub async fn file_push(
    lxd: &LxdClient,
    container: &LxdContainerName,
    host_file: &Path,
    container_file: &Path,
) -> Result<()> {
    let from = host_file
        .display()
        .to_string();

    let to = {
        let file = container_file
            .display()
            .to_string();

        format!("{}/{}", container, file)
    };

    cmds::invoke(lxd, &[
        "file".to_string(),
        "push".to_string(),
        from,
        to,
    ]).await?;

    Ok(())
}
