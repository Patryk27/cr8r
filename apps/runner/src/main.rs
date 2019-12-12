#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

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
        .context(error::FailedToStart)?;

    let config = config::load()?;
    let sandbox_provider = SandboxProvider::new()?;

    info!("Checking for stale containers from previous sessions");

    let client = Client::connect(config.controller.address)
        .await?;

    let client = SessionClient::start(config.runner.name, config.controller.secret, client)
        .await?;

    info!("{}", "🚀 We are ready to accept commands".green());

    SystemHeartbeater::spawn(
        client.clone()
    );

    SystemActor::new(sandbox_provider, client)
        .main()
        .await
}