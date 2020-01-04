#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use anyhow::{Context, Result};
use colored::Colorize;
use log::*;

use lib_error::PrintableError;
use lib_interop::client::ControllerClient;
use lib_sandbox::SandboxProvider;

use self::{
    backend::*,
    core::*,
};

mod backend;
mod core;

#[tokio::main]
async fn main() {
    let result = try {
        lib_log::init()
            .context("Could not initialize logging facility")?;

        let config = Config::load()
            .context("Could not load configuration from `runner.yaml``")?;

        let sandbox_provider = SandboxProvider::new();

        let client = ControllerClient::connect(config.controller.address, config.controller.secret)
            .await
            .context("Could not connect to controller")?;

        let client = SessionClient::start(config.runner.name, client)
            .await
            .context("Could not connect to controller")?;

        info!("{}", "🚀 We are ready to accept commands".green());

        SystemHeartbeater::new(
            client.clone()
        );

        SystemActor::new(config.sandbox, sandbox_provider, client)
            .main()
            .await?
    }: Result<()>;

    if let Err(err) = result {
        err.print();
        exit(1);
    }
}