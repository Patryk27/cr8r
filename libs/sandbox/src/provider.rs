use std::convert::TryInto;
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

    pub async fn create(&self, name: String) -> Result<Sandbox> {
        let name = name.try_into()?;
        let containers = self.lxd.list()?;

        for container in containers {
            if &container.name == &name {
                self.lxd
                    .delete(&name)?
                    .wait()
                    .await?;

                break;
            }
        }

        Ok(Sandbox::new(
            self.lxd.clone(), name,
        ))
    }
}