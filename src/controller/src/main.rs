#![feature(box_syntax)]

use std::path::PathBuf;

use actix::Actor;

pub use error::Result;

mod config;
mod error;
mod http;
mod modules;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        &PathBuf::from("controller.yaml")
    )?;

    // Prepare system
    let system = actix::System::new("cr8r");

    // Launch the system actor (it's a non-blocking call)
    let system_addr = modules::System::new(config.controller.runner_secret)
        .start();

    // Launch the HTTP server actor (it's a non-blocking call)
    http::HttpServer::new(&config.controller.bind, system_addr)
        .start()?;

    // Takeoff!
    Ok(system.run()?)
}
