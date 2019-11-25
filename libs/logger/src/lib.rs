use std::error::Error;
use std::io;

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

pub fn init() -> Result<(), Box<dyn Error>> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::BrightCyan)
        .trace(Color::BrightCyan);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .level_for("actix_server", LevelFilter::Error)
        .level_for("tokio_reactor", LevelFilter::Error)
        .chain(io::stdout())
        .apply()?;

    Ok(())
}