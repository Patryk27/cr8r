#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use anyhow::{Context, Result};
use colored::Colorize;
use log::*;

use lib_interop::client::ControllerClient;
use lib_sandbox::SandboxProvider;

mod cargo;
mod core;
mod experiment;
mod session;
mod system;

#[tokio::main]
async fn main() {
    use self::{
        core::*,
        session::*,
        system::*,
    };

    let result = try {
        lib_log::init()
            .context("Could not initialize logging facility")?;

        let config = Config::load()
            .context("Could not load configuration from `runner.yaml``")?;

        let sandbox_provider = SandboxProvider::new();

        let client = ControllerClient::connect(config.controller.address, config.controller.secret)
            .await
            .context("Could not connect to controller")?;

        let client = Session::open(client, config.runner.name)
            .await
            .context("Could not connect to controller")?;

        info!("{}", "🚀 We are ready to accept commands".green());

        System {
            sandbox_config: config.sandbox,
            sandbox_provider,
            session: client,
        }.start().await?
    }: Result<()>;

    if let Err(err) = result {
        eprintln!("{}", lib_ui::Error::new(&err));

        exit(1);
    }
}