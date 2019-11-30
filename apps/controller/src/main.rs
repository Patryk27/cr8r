#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::path::PathBuf;

use snafu::ResultExt;

pub use self::core::*;

mod system;
mod core;
mod interface;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()
        .context(error::FailedToConfigure)?;

    let config = config::load(
        &PathBuf::from("controller.yaml")
    )?;

    let system = system::start(
        config.controller.runner_secret, config.ecosystem,
    );

    interface::start(
        config.controller.listen, system,
    ).await?;

    Ok(())
}