// @todo ensure controller's protocol version matches ours

#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use anyhow::{Context, Result};
use colored::Colorize;
use structopt::StructOpt;

pub use self::{
    commands::*,
    config::*,
    system::*,
};

mod commands;
mod config;
mod system;

#[macro_use]
mod ui;

#[tokio::main]
async fn main() {
    let result = try {
        let cmd = Command::from_args();

        let config = config::load()
            .context("Failed to load configuration")?;

        let system = System::new(config);

        cmd.run(system)
            .await
    }: Result<_>;

    if let Err(err) = result {
        println!("{} {}", "Error:".red(), err);
        exit(1);
    }
}
