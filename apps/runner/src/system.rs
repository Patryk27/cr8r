use anyhow::*;
use colored::Colorize;
use log::*;

use lib_core_ui::Logo;
use lib_interop::connection::Connection;
use lib_sandbox::SandboxProvider;

use crate::build;
use crate::config::Config;
use crate::rpc::Session;

pub use self::{
    attachment::*,
    attachment_store::*,
    dispatcher::*,
    executor::*,
    logger::*,
};

mod attachment;
mod attachment_store;
mod dispatcher;
mod executor;
mod logger;

pub async fn start(config: Config) -> Result<()> {
    let sandbox_provider = SandboxProvider::new(config.sandbox)
        .await
        .context("Could not initialize sandbox")?;

    let conn = Connection::new(config.controller.address.clone(), config.controller.secret)
        .await
        .context("Could not connect to the controller")?;

    let attachment_store = AttachmentStore::new(config.attachments, conn.attachments())
        .await
        .context("Could not initialize attachment store")?;

    let session = Session::new(conn, config.runner.name.into())
        .await
        .context("Could not open session")?;

    Logo {
        app: build::PKG_NAME,
        version: build::PKG_VERSION,
        commit: build::GIT_VERSION.unwrap(),
    }.log();

    info!("🚀 Connected to: {}", config.controller.address.green());

    info!(
        "Authorized as: id={}, name={}",
        session.runner_id().to_string().green(),
        session.runner_name().to_string().green(),
    );

    let dispatcher = Dispatcher {
        sandbox_provider,
        attachment_store,
        session,
    }.start();

    dispatcher.await
}