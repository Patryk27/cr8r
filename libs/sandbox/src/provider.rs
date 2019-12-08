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

    pub fn provide(&self, name: String) -> Sandbox {
        Sandbox::new(self.lxd.clone(), name)
    }
}