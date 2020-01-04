// @todo ensure controller's protocol version matches ours

#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use anyhow::{Context, Result};
use structopt::StructOpt;

use lib_error::PrintableError;

use self::{
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

        let config = Config::load()
            .context("Could not load configuration from `client.yaml`")?;

        let system = System::new(config);

        cmd.run(system)
            .await?
    }: Result<()>;

    if let Err(err) = result {
        err.print();
        exit(1);
    }
}
