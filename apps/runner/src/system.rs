use anyhow::*;
use colored::Colorize;
use log::*;

use lib_core_ui::Logo;
use lib_interop::connection::Connection;
use lib_sandbox::SandboxProvider;

use crate::build;
use crate::rpc::{RpcConfig, Session};

pub use self::{
    config::*,
    modules::*,
};

mod config;
mod modules;

pub async fn start(rpc_cfg: RpcConfig, system_cfg: SystemConfig) -> Result<()> {
    let sandbox_provider = SandboxProvider::new(
        system_cfg.sandbox,
    ).await.context("Could not initialize sandbox")?;

    let conn = Connection::new(
        rpc_cfg.address.clone(),
        rpc_cfg.secret,
    ).await.context("Could not connect to the controller")?;

    let attachment_store = AttachmentStore::new(
        system_cfg.attachments,
        conn.attachments(),
    ).await.context("Could not initialize attachment store")?;

    let session = Session::new(
        conn,
        system_cfg.runner.name.into(),
    ).await.context("Could not open session")?;

    Logo {
        app: build::PKG_NAME,
        version: build::PKG_VERSION,
        commit: build::GIT_VERSION.unwrap(),
    }.log();

    info!(
        "🚀 Connected to: {}",
        rpc_cfg.address.green(),
    );

    info!(
        "Authorized as: id={}, name={}",
        session.runner_id().to_string().green(),
        session.runner_name().to_string().green(),
    );

    HeartbeatSyncer::new(
        session.clone()
    );

    let dispatcher = Dispatcher {
        sandbox_provider,
        attachment_store,
        session,
    }.start();

    dispatcher.await
}