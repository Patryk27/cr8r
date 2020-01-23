use anyhow::{anyhow, Result};
use lib_process::Process;

use crate::LxdClient;

pub async fn invoke(lxd: &LxdClient, args: &[String]) -> Result<String> {
    invoke_ex(lxd, args, true)
        .await
}

pub async fn invoke_silent(lxd: &LxdClient, args: &[String]) -> Result<String> {
    invoke_ex(lxd, args, false)
        .await
}

async fn invoke_ex(lxd: &LxdClient, args: &[String], listen: bool) -> Result<String> {
    let mut output = String::new();

    let status = Process::new(&lxd.path)
        .args(args)
        .listener(|line| {
            output.push_str(&line);
            output.push('\n');

            if listen {
                if let Some(handler) = &lxd.listener.on_output {
                    handler(line);
                }
            }
        })
        .spawn()
        .await?;

    if status.success() {
        Ok(output)
    } else {
        Err(anyhow!("Previous command returned a non-zero exit code"))
    }
}