use anyhow::*;

use crate::{LxdConnector, LxdContainerName, LxdDeviceDef, LxdDeviceName};

pub async fn config_device_add(
    conn: &LxdConnector,
    cname: &LxdContainerName,
    dev_name: LxdDeviceName,
    dev_def: LxdDeviceDef,
) -> Result<()> {
    let mut args = vec![
        "device".to_string(),
        "add".to_string(),
        cname.to_string(),
        dev_name.to_string(),
    ];

    match dev_def {
        LxdDeviceDef::Disk { source, path } => {
            args.push("disk".to_string());
            args.push(format!("source={}", source));
            args.push(format!("path={}", path));
        }
    };

    conn.invoke(&args).await?;

    Ok(())
}