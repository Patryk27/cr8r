#![feature(try_blocks)]
#![feature(type_ascription)]

use colored::Colorize;
use log::*;
use snafu::ResultExt;

use self::{
    backend::*,
    core::*,
};

mod backend;
mod core;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()
        .context(error::FailedToConfigure)?;

    let config = config::load()?;
    let client = Client::connect(config.controller.address).await?;
    let session_client = SessionClient::start(config.runner.name, config.controller.secret, client).await?;

    info!("{}", "🚀 We are ready to accept commands".green());

    Heartbeat::spawn(
        session_client.clone()
    );

    RunnerActor::new(session_client)
        .start()
        .await
}