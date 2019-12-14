use std::borrow::Cow;
use std::env;
use std::time::Duration;

use snafu::ResultExt;
use tokio::timer;

use lib_sandbox_lxd::LxdImageName;

use crate::{error, Result, Sandbox, SandboxMount};

impl Sandbox {
    pub async fn initialize(&mut self, system: &str, toolchain: &str) -> Result<()> {
        self.launch_container(system)
            .await
            .context(error::CouldntLaunchContainer)?;

        self.forward_ssh_agent()
            .await
            .context(error::CouldntForwardSshAgent)?;

        self.wait_for_network()
            .await
            .context(error::CouldntWaitForNetwork)?;

        self.install_toolchain(toolchain)
            .await
            .context(error::CouldntInstallToolchain)?;

        Ok(())
    }

    async fn launch_container(&mut self, system: &str) -> Result<()> {
        let system: LxdImageName = system
            .to_string()
            .into();

        self.invoke(|lxd| lxd.launch(&system, &self.container))
            .await
    }

    async fn forward_ssh_agent(&mut self) -> Result<()> {
        // @todo extract to a distinct fn
        let ssh_sock = env::var("SSH_AUTH_SOCK")
            .map_err(|_| snafu::NoneError)
            .context(error::MissingEnvVariable { name: Cow::Borrowed("SSH_AUTH_SOCK") })?;

        self.add_mount(SandboxMount::File {
            host: ssh_sock,
            sandbox: "/tmp/ssh-agent".to_string(),
        }).await?;

        self.add_env("SSH_AUTH_SOCK", "/tmp/ssh-agent".to_string())
            .await
    }

    async fn wait_for_network(&mut self) -> Result<()> {
        // Wait a bit before systemd gets initialized; otherwise we won't be able to invoke `systemctl`
        timer::delay_for(Duration::from_millis(1000))
            .await;

        self.exec("systemctl start network-online.target")
            .await
    }

    async fn install_toolchain(&mut self, toolchain: &str) -> Result<()> {
        // LXD's default Ubuntu images do not contain `cc`, so compiling any Cargo program would fail if we didn't pull
        // `cmake`
        self.exec("apt update && apt install cmake -y")
            .await?;

        self.exec(&format!("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain {} --profile minimal", toolchain))
            .await?;

        // We cannot modify `~/.bashrc` or `~/.bash_profile`, because our `self.exec()` ignores it by design

        let path = self.get_env("PATH").await?;
        let path = format!("{}:/root/.cargo/bin", path);

        self.add_env("PATH", path)
            .await
    }
}