use anyhow::*;
use colored::Colorize;
use log::*;

use lib_sandbox::SandboxProvider;

use crate::config::Config;
use crate::rpc::Session;

pub use self::{
    dispatcher::*,
    executor::*,
    logger::*,
};

mod dispatcher;
mod executor;
mod logger;

pub async fn start(config: Config) -> Result<()> {
    unimplemented!()
//    let sandbox_provider = SandboxProvider::new();
//
//    let client = ControllerClient::connect(config.controller.address, config.controller.secret)
//        .await
//        .context("Could not connect to the controller")?;
//
//    let session = Session::open(client, config.runner.name)
//        .await
//        .context("Could not open session")?;
//
//    let dispatcher = Dispatcher {
//        sandbox_config: config.sandbox,
//        sandbox_provider,
//        session,
//    }.start();
//
//    info!("{}", "🚀 We are ready to accept commands".green());
//
//    dispatcher.await
}