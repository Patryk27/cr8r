use std::error::Error;
use std::path::PathBuf;

use actix::{Actor, Arbiter, StreamHandler, System};
use actix::io::SinkWrite;
use actix::prelude::{Future, Stream};
use actix_web::client::Client;
use log::*;

mod config;
mod error;
mod modules;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("tokio_reactor", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .apply()?;

    // Load configuration
    let config = config::from_file(
        &PathBuf::from("runner.yaml")
    )?;

    // Prepare system
    let system = System::new("cr8r");

    info!("Connecting to controller at `{}`, this may take a few seconds.", config.controller.address);

    Arbiter::spawn_fn(move || {
        Client::new()
            .ws(&config.controller.address)
            .connect()
            .map_err(|err| {
                error!("Could not connect to controller: {:?}", err);
            })
            .map(|(_, stream)| {
                info!("Connection acquired, authenticating.");

                let (sink, stream) = stream.split();

                modules::SystemActor::create(|ctx| {
                    modules::SystemActor::add_stream(stream, ctx);

                    modules::SystemActor::new(
                        config.runner.name,
                        config.controller.secret,
                        modules::RunnerSocket::new(SinkWrite::new(sink, ctx)),
                    )
                });
            })
    });

    // Takeoff!
    Ok(system.run()?)
}
