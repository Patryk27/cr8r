use log::*;

use crate::engines::ShellSandboxEngine;
use crate::Result;

pub async fn destroy(_: &mut ShellSandboxEngine) -> Result<()> {
    debug!("Executing: destroy()");

    Ok(())
}