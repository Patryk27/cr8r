use snafu::ResultExt;

use lib_process::Process;

use crate::{Error, error, LxdClient, Result};

pub async fn invoke(lxd: &LxdClient, args: &[String]) -> Result<String> {
    let mut output = String::new();

    let status = Process::new(&lxd.path)
        .args(args)
        .listener(box |line| {
            output.push_str(&line);
            output.push('\n');

            if let Some(handler) = &lxd.listener.on_output {
                handler(line);
            }
        })
        .spawn()
        .await
        .context(error::CommandNotStarted)?;

    if status.success() {
        Ok(output)
    } else {
        Err(Error::CommandFailed)
    }
}