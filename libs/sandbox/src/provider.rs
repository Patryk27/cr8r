use crate::{Result, Sandbox, SandboxConfig};

pub struct SandboxProvider;

impl SandboxProvider {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, definition: SandboxConfig) -> Result<Sandbox> {
        use crate::engines::*;

        let engine = match definition {
            SandboxConfig::Lxd(definition) => {
                box LxdEngine::create(definition)
                    .await? as _
            }

            SandboxConfig::Shell(definition) => {
                box ShellEngine::create(definition)
                    .await? as _
            }
        };

        Ok(Sandbox::new(engine))
    }
}