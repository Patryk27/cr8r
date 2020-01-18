use std::path::Path;

use anyhow::Result;

use crate::{commands, LxdClient, LxdContainerName};

pub async fn file_pull(
    lxd: &LxdClient,
    container: &LxdContainerName,
    container_file: &Path,
    host_file: &Path,
) -> Result<()> {
    let from = {
        let container_file = container_file
            .display()
            .to_string();

        format!("{}/{}", container, container_file)
    };

    let to = host_file
        .display()
        .to_string();

    commands::invoke(lxd, &[
        "file".to_string(),
        "pull".to_string(),
        from,
        to,
    ]).await?;

    Ok(())
}