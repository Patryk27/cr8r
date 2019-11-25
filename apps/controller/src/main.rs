#![feature(try_blocks)]
#![feature(type_ascription)]

use snafu::ResultExt;

pub use self::{
    core::*,
};

mod backend;
mod core;
mod frontend;

fn main() -> Result<()> {
    lib_logger::init()
        .context(error::FailedToConfigure)?;

    let config = config::load()?;

    // Prepare the backend side (actors and stuff)
    let (actix, system) = backend::start(config.ecosystem, config.controller.runner_secret);

    // Prepare the frontend side (HTTP server and stuff)
    frontend::start(config.controller.bind, system)?;

    // Takeoff!
    actix.run()
        .context(error::FailedToStart)?;

    Ok(())
}
