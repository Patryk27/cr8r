use std::path::Path;

use anyhow::*;

use crate::{LxdConnector, LxdContainerName};

pub async fn file_push(
    conn: &LxdConnector,
    cname: &LxdContainerName,
    from: &Path,
    to: &Path,
) -> Result<()> {
    let from = from
        .display()
        .to_string();

    let to = {
        let file = to
            .display()
            .to_string();

        format!("{}/{}", cname, file)
    };

    conn.invoke_silent(&[
        "file".to_string(),
        "push".to_string(),
        from,
        to,
    ]).await?;

    Ok(())
}
