#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::path::PathBuf;

use colored::Colorize;
use log::*;
use snafu::ResultExt;

use lib_sandbox::SandboxProvider;

use self::{
    backend::*,
    core::*,
};

mod backend;
mod core;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()
        .context(error::CouldntStart)?;

    let config = config::load(
        &PathBuf::from("runner.yaml")
    )?;

    let sandbox_provider = SandboxProvider::new();

    let client = Client::connect(config.controller.address)
        .await?;

    let client = SessionClient::start(config.runner.name, config.controller.secret, client)
        .await?;

    info!("{}", "🚀 We are ready to accept commands".green());

    SystemHeartbeater::new(
        client.clone()
    );

    SystemActor::new(config.sandbox, sandbox_provider, client)
        .main()
        .await
}