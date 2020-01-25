use std::io;

use anyhow::*;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

pub fn init() -> Result<()> {
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
        .level_for("h2", LevelFilter::Error)
        .level_for("hyper", LevelFilter::Error)
        .level_for("tokio_reactor", LevelFilter::Error)
        .level_for("tower", LevelFilter::Error)
        .level_for("tower_buffer", LevelFilter::Error)
        .chain(io::stdout())
        .apply()?;

    Ok(())
}