use crate::{cmds, LxdClient, LxdContainerName, Result};

pub async fn exec(lxd: &LxdClient, container: &LxdContainerName, args: &[&str]) -> Result<String> {
    let mut invoke_args = vec![
        "exec".to_string(),
        container.to_string(),
        "--".to_string(),
    ];

    for arg in args {
        invoke_args.push(arg.to_string());
    }

    cmds::invoke(lxd, &invoke_args)
        .await
}