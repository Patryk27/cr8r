use lib_lxd::LxdImageName;
use snafu::ResultExt;
use std::borrow::Cow;
use std::env;
use std::time::Duration;
use tokio::timer;

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


}