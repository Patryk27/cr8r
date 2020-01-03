#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

use std::path::PathBuf;

use colored::Colorize;
use log::*;
use snafu::ResultExt;

use lib_interop::client::ControllerClient;
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

    let client = ControllerClient::connect(config.controller.address, config.controller.secret)
        .await
        .unwrap();

    let client = SessionClient::start(config.runner.name, client)
        .await?;

    info!("{}", "🚀 We are ready to accept commands".green());

    SystemHeartbeater::new(
        client.clone()
    );

    SystemActor::new(config.sandbox, sandbox_provider, client)
        .main()
        .await
}