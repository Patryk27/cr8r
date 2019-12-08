use std::convert::TryInto;
use std::time::Duration;

use tokio::timer;

use lib_lxd::LxdImageName;

use crate::{Result, Sandbox, SandboxMount};

impl Sandbox {
    pub async fn initialize(&mut self, system: &str, toolchain: &str) -> Result<()> {
        let system: LxdImageName = system
            .to_string()
            .try_into()
            .unwrap();

        // Launch the container
        self.invoke(|lxd| lxd.launch(&system, &self.container))
            .await?;

        // Forward SSH agent
        self.add_mount(SandboxMount::File {
            host: std::env::var("SSH_AUTH_SOCK").unwrap(),
            sandbox: "/tmp/ssh-agent".to_string(),
        }).await?;

        self.add_env("SSH_AUTH_SOCK", "/tmp/ssh-agent".to_string())
            .await?;

        // Wait a bit before systemd gets initialized; otherwise we won't be able to perform any `systemctl` calls
        timer::delay_for(Duration::from_millis(1000)).await;

        // Wait until network gets initialized
        self.exec("systemctl start network-online.target")
            .await?;

        // Install `rustup`
        self.exec("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .await?;

        self.exec("echo 'export PATH=\"$HOME/.cargo/bin:$PATH\"' >> ~/.bash_profile")
            .await?;

        // Install the selected toolchain
        // @todo

        Ok(())
    }
}