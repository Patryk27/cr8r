use anyhow::*;

use crate::{LxdConnector, LxdContainerName};

pub async fn exec(
    conn: &LxdConnector,
    cname: &LxdContainerName,
    proc_args: &[&str],
) -> Result<String> {
    let mut args = vec![
        "exec".to_string(),
        cname.to_string(),
        "--".to_string(),
    ];

    for arg in proc_args {
        args.push(arg.to_string());
    }

    conn.invoke(&args).await
}