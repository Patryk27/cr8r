use std::path::Path;

use anyhow::*;

use crate::{LxdConnector, LxdContainerName};

pub async fn file_pull(
    conn: &LxdConnector,
    cname: &LxdContainerName,
    from: &Path,
    to: &Path,
) -> Result<()> {
    let from = {
        let file = from
            .display()
            .to_string();

        format!("{}/{}", cname, file)
    };

    let to = to
        .display()
        .to_string();

    conn.invoke_silent(&[
        "file".to_string(),
        "pull".to_string(),
        from,
        to,
    ]).await?;

    Ok(())
}