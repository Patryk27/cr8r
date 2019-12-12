use std::sync::Arc;

use crate::{LxdClient, Result, Sandbox};

pub struct SandboxProvider {
    lxd: Arc<LxdClient>,
}

impl SandboxProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {
            lxd: Arc::new(LxdClient::autodetect()?),
        })
    }

    pub fn gc(&self) -> Result<()> {
        let containers = self.lxd.list()?;

        for container in containers {
            if container.name.as_str().starts_with("cr8r-") {
                self.lxd
                    .delete(&container.name)?
                    .wait_sync()?;
            }
        }

        Ok(())
    }

    pub fn provide(&self, name: String) -> Sandbox {
        Sandbox::new(self.lxd.clone(), name)
    }
}