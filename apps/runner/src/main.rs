#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use actix::{Arbiter, System};
use actix::prelude::Future;
use actix_web::client::Client;
use log::*;
use snafu::ResultExt;

pub use self::{
    config::Config,
    error::{Error, Result, StdResult},
    runner::*,
};

mod config;
mod error;
mod runner;

fn main() -> Result<()> {
    lib_logger::init()
        .context(error::FailedToConfigure)?;

    let config = config::load()?;

    let actix = System::new("cr8r");

    info!("Connecting to controller at `{}`.", config.controller.address);

    Arbiter::spawn_fn(move || {
        Client::new()
            .ws(&config.controller.address)
            .connect()
            .map_err(|err| {
                error!("Could not connect to controller: {:?}", err);
                error!("Shutting down.");

                exit(1);
            })
            .map(|(_, stream)| {
                Runner::spawn(
                    config.runner.name,
                    config.controller.secret,
                    stream,
                )
            })
    });

    // Takeoff!
    actix.run()
        .context(error::FailedToStart)?;

    Ok(())
}