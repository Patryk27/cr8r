use anyhow::*;
use log::*;

use crate::engines::ShellSandboxEngine;

pub async fn destroy(_: &mut ShellSandboxEngine) -> Result<()> {
    trace!("Executing: destroy()");

    Ok(())
}