use log::*;

use crate::engines::ShellEngine;
use crate::Result;

pub async fn destroy(_: &mut ShellEngine) -> Result<()> {
    debug!("destroy");

    Ok(())
}