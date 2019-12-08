use std::convert::TryInto;

use lib_lxd::LxdContainerConfig;

use crate::{Result, Sandbox, SandboxMount};

impl Sandbox {
    pub async fn add_mount(&mut self, mount: SandboxMount) -> Result<()> {
        let name = format!("cr8r-mount-{}", self.mount_idx)
            .try_into()
            .unwrap();

        self.mount_idx += 1;

        self.invoke(|lxd| {
            lxd.config(&self.container, LxdContainerConfig::AddDevice {
                name,
                def: mount.into_device_def(),
            })
        }).await
    }
}