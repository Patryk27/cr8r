use anyhow::*;
use colored::Colorize;
use log::*;

use lib_core_ui::Logo;
use lib_interop::connection::ControllerConnection;
use lib_sandbox::SandboxProvider;

use crate::build;
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
    let sandbox_provider = SandboxProvider::new(config.sandbox)
        .await
        .context("Could not initialize sandbox")?;

    let conn = ControllerConnection::new(config.controller.address.clone(), config.controller.secret)
        .await
        .context("Could not connect to the controller")?;

    let session = Session::open(conn, config.runner.name)
        .await
        .context("Could not open session")?;

    Logo {
        app: build::PKG_NAME,
        version: build::PKG_VERSION,
        commit: build::GIT_VERSION.unwrap(),
    }.log();

    info!("🚀 Connected to: {}", config.controller.address.green());

    let dispatcher = Dispatcher {
        sandbox_provider,
        session,
    }.start();

    dispatcher.await
}