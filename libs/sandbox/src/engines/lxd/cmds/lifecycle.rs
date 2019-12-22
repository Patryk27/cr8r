use std::time::Duration;

use tokio::time;

use lib_lxd::{LxdContainerConfig, LxdDeviceDef, LxdListener};

use crate::{LxdEngine, Result, SandboxListener};
use crate::engines::lxd::cmds;

pub async fn init(engine: &mut LxdEngine, mut listener: SandboxListener) -> Result<()> {
    engine.lxd.set_listener(LxdListener {
        on_output: listener.on_command_output.take(),
    });

    engine.listener = listener;

    delete_stale_container(engine)
        .await?;

    launch_container(engine)
        .await?;

    forward_ssh_agent(engine)
        .await?;

    wait_for_network(engine)
        .await?;

    install_rustup(engine)
        .await?;

    Ok(())
}

pub async fn destroy(engine: &mut LxdEngine) -> Result<()> {
    engine.lxd
        .delete(&engine.container)
        .await?;

    Ok(())
}

async fn delete_stale_container(engine: &mut LxdEngine) -> Result<()> {
    let found_stale_container = engine.lxd
        .list()
        .await?
        .into_iter()
        .any(|container| container.name == engine.container);

    if found_stale_container {
        engine.lxd
            .delete(&engine.container)
            .await?;
    }

    Ok(())
}

async fn launch_container(engine: &mut LxdEngine) -> Result<()> {
    engine.lxd
        .launch(&engine.image, &engine.container)
        .await?;

    Ok(())
}

async fn forward_ssh_agent(engine: &mut LxdEngine) -> Result<()> {
    let ssh_sock = cmds::get_host_env("SSH_AUTH_SOCK")?;

    engine.lxd.config(&engine.container, LxdContainerConfig::AddDevice {
        name: format!("{}-ssh-auth-sock", engine.container.as_str())
            .parse()
            .unwrap(),

        def: LxdDeviceDef::Disk {
            source: ssh_sock,
            path: "/tmp/ssh-agent".to_string(),
        },
    }).await?;

    cmds::set_env(engine, "SSH_AUTH_SOCK", "/tmp/ssh-agent")
        .await
}

async fn wait_for_network(engine: &mut LxdEngine) -> Result<()> {
    // Wait a bit before systemd gets initialized; otherwise we won't be able to invoke `systemctl`
    time::delay_for(Duration::from_millis(1000))
        .await;

    cmds::exec(engine, "systemctl start network-online.target")
        .await
}

async fn install_rustup(engine: &mut LxdEngine) -> Result<()> {
    // LXD's default Ubuntu images do not contain `cc`, so compiling any Cargo program would fail if we didn't pull
    // `cmake`
    cmds::exec(engine, "apt update && apt install cmake -y")
        .await?;

    cmds::exec(engine, "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal")
        .await?;

    // @todo describe
    let path = cmds::get_env(engine, "PATH").await?;
    let path = format!("{}:/root/.cargo/bin", path);

    cmds::set_env(engine, "PATH", &path)
        .await
}