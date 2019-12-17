use crate::{cmds, LxdClient, LxdContainerConfig, LxdContainerName, Result};

pub async fn config(lxd: &LxdClient, container: &LxdContainerName, config: LxdContainerConfig) -> Result<()> {
    let mut args = vec![
        "config".to_string(),
    ];

    args.extend(config.into_args(container));

    cmds::invoke(lxd, &args)
        .await?;

    Ok(())
}