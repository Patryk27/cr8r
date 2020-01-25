use anyhow::*;
use log::*;

use crate::engines::ShellSandboxEngine;

pub async fn destroy(_: &mut ShellSandboxEngine) -> Result<()> {
    debug!("Executing: destroy()");

    Ok(())
}