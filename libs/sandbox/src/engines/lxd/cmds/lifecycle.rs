use std::time::Duration;

use tokio::timer;

use crate::{LxdEngine, Result, SandboxEngine, SandboxListener, SandboxMount};
use std::env;

pub async fn init(engine: &mut LxdEngine, listener: SandboxListener) -> Result<()> {
    engine.listener = listener;

    unimplemented!()
}

pub async fn destroy(engine: &mut LxdEngine) -> Result<()> {
    unimplemented!()
}

async fn launch_container(engine: &mut LxdEngine) -> Result<()> {
    self.invoke(|lxd| lxd.launch(&system, &self.container))
        .await
}

async fn forward_ssh_agent(engine: &mut LxdEngine) -> Result<()> {
    // @todo extract to a distinct fn
    let ssh_sock = env::var("SSH_AUTH_SOCK")
        .map_err(|_| snafu::NoneError)
        .context(error::MissingEnvVariable { name: Cow::Borrowed("SSH_AUTH_SOCK") })?;

    engine.mount(SandboxMount::File {
        host: ssh_sock,
        sandbox: "/tmp/ssh-agent".to_string(),
    }).await?;

    engine.add_env("SSH_AUTH_SOCK", "/tmp/ssh-agent".to_string())
        .await
}

async fn wait_for_network(engine: &mut LxdEngine) -> Result<()> {
    // Wait a bit before systemd gets initialized; otherwise we won't be able to invoke `systemctl`
    timer::delay_for(Duration::from_millis(1000))
        .await;

    engine.exec("systemctl start network-online.target")
        .await
}

async fn install_toolchain(engine: &mut LxdEngine, toolchain: &str) -> Result<()> {
    // LXD's default Ubuntu images do not contain `cc`, so compiling any Cargo program would fail if we didn't pull
    // `cmake`
    engine.exec("apt update && apt install cmake -y")
        .await?;

    engine.exec(&format!("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain {} --profile minimal", toolchain))
        .await?;

    // We cannot modify `~/.bashrc` or `~/.bash_profile`, because our `self.exec()` ignores it by design

    let path = engine.get_env("PATH").await?;
    let path = format!("{}:/root/.cargo/bin", path);

    engine.add_env("PATH", path)
        .await
}