use anyhow::{Context, Result};
use log::*;
use tokio::time;

use lib_lxd::{LxdContainerConfig, LxdDeviceDef, LxdListener};

use crate::engines::LxdEngine;
use crate::SandboxListener;

pub async fn init(engine: &mut LxdEngine, mut listener: SandboxListener) -> Result<()> {
    debug!("init");

    engine.client.set_listener(LxdListener {
        on_output: listener.on_command_output.take(),
    });

    engine.listener = listener;

    delete_stale_container(engine)
        .await
        .context("Could not delete stale container")?;

    launch_container(engine)
        .await
        .context("Could not launch new container")?;

    forward_ssh_agent(engine)
        .await
        .context("Could not forward SSH agent")?;

    wait_for_network(engine)
        .await
        .context("Could not wait for network")?;

    install_rustup(engine)
        .await
        .context("Could not install `rustup`")?;

    Ok(())
}

async fn delete_stale_container(engine: &mut LxdEngine) -> Result<()> {
    let found_stale_container = engine.client
        .list()
        .await?
        .into_iter()
        .any(|container| container.name == engine.container);

    if found_stale_container {
        engine.client
            .delete(&engine.container)
            .await?;
    }

    Ok(())
}

async fn launch_container(engine: &mut LxdEngine) -> Result<()> {
    engine.client
        .launch(&engine.image, &engine.container)
        .await?;

    Ok(())
}

async fn forward_ssh_agent(engine: &mut LxdEngine) -> Result<()> {
    let ssh_sock = super::get_host_env("SSH_AUTH_SOCK")?;

    engine.client.config(&engine.container, LxdContainerConfig::AddDevice {
        name: format!("{}-ssh-auth-sock", engine.container.as_str())
            .parse()?,

        def: LxdDeviceDef::Disk {
            source: ssh_sock,
            path: "/tmp/ssh-agent".to_string(),
        },
    }).await?;

    super::set_env(engine, "SSH_AUTH_SOCK", "/tmp/ssh-agent")
        .await
}

async fn wait_for_network(engine: &mut LxdEngine) -> Result<()> {
    // Wait a bit before systemd gets initialized; otherwise we won't be able to invoke `systemctl`
    time::delay_for(time::Duration::from_millis(1000))
        .await;

    super::exec(engine, "systemctl start network-online.target")
        .await
}

async fn install_rustup(engine: &mut LxdEngine) -> Result<()> {
    // LXD's default Ubuntu images do not contain `cc`, so compiling any Cargo program would fail if we didn't pull
    // `cmake`
    super::exec(engine, "apt update && apt install cmake -y")
        .await?;

    super::exec(engine, "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal")
        .await?;

    // @todo describe
    let path = super::get_env(engine, "PATH").await?;
    let path = format!("{}:/root/.cargo/bin", path);

    super::set_env(engine, "PATH", &path)
        .await
}