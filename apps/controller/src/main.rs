#![feature(try_blocks)]
#![feature(type_ascription)]

use snafu::ResultExt;

pub use self::core::*;

mod backend;
mod core;
mod frontend;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()
        .context(error::FailedToConfigure)?;

    let config = config::load()?;

    // Prepare the backend side (actors and stuff)
    let system = backend::start();

    // Prepare the frontend side (RPC server and stuff) - and takeoff!
    frontend::start(config.controller.bind, system).await?;

    Ok(())
}