use anyhow::*;
use log::*;
use tokio::time;

use lib_lxd::{LxdContainerConfig, LxdDeviceDef, LxdListener};

use crate::engines::LxdSandboxEngine;
use crate::SandboxListener;

pub async fn init(engine: &mut LxdSandboxEngine, listener: SandboxListener) -> Result<()> {
    trace!("Executing: init()");

    engine.client.set_listener(LxdListener {
        on_output: listener.on_command_output,
    });

    engine.listener = SandboxListener::default();

    delete_stale_container(engine)
        .await
        .context("Could not delete stale container")?;

    launch_container(engine)
        .await
        .context("Could not launch new container")?;

    if engine.config.forward_ssh {
        forward_ssh_agent(engine)
            .await
            .context("Could not forward SSH agent")?;
    }

    if engine.config.wait_for_network {
        wait_for_network(engine)
            .await
            .context("Could not wait for network")?;
    }

    install_rustup(engine)
        .await
        .context("Could not install `rustup`")?;

    Ok(())
}

async fn delete_stale_container(engine: &mut LxdSandboxEngine) -> Result<()> {
    trace!(".. checking for a stale container");

    let found_stale_container = engine.client
        .list()
        .await?
        .into_iter()
        .any(|container| container.name == engine.config.container);

    if found_stale_container {
        trace!(".. .. ok, found - deleting it");

        engine.client
            .delete(&engine.config.container)
            .await?;
    } else {
        trace!(".. .. ok, not found");
    }

    Ok(())
}

async fn launch_container(engine: &mut LxdSandboxEngine) -> Result<()> {
    trace!(".. launching new container");

    engine.client
        .launch(&engine.config.image, &engine.config.container)
        .await?;

    Ok(())
}

async fn forward_ssh_agent(engine: &mut LxdSandboxEngine) -> Result<()> {
    trace!(".. forwarding SSH agent");

    let ssh_sock = super::get_host_env("SSH_AUTH_SOCK")?;

    trace!(".. .. SSH_SOCK = {}", ssh_sock);

    engine.client.config(&engine.config.container, LxdContainerConfig::AddDevice {
        name: format!("{}-ssh-auth-sock", engine.config.container.as_str())
            .parse()?,

        def: LxdDeviceDef::Disk {
            source: ssh_sock,
            path: "/tmp/ssh-agent".to_string(),
        },
    }).await?;

    super::set_env(engine, "SSH_AUTH_SOCK", "/tmp/ssh-agent")
        .await
}

async fn wait_for_network(engine: &mut LxdSandboxEngine) -> Result<()> {
    trace!(".. waiting for network");

    // Wait a bit before systemd gets initialized; otherwise we won't be able to invoke `systemctl`
    time::delay_for(time::Duration::from_millis(1000))
        .await;

    super::exec(engine, "systemctl start network-online.target")
        .await
}

async fn install_rustup(engine: &mut LxdSandboxEngine) -> Result<()> {
    trace!(".. installing `rustup`");

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