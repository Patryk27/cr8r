use crate::{Result, Sandbox, SandboxDef, SandboxListener};

pub struct SandboxProvider;

impl SandboxProvider {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, def: SandboxDef) -> Result<Sandbox> {
        use crate::engines::*;

        let engine = match def {
            SandboxDef::Lxd { container, image } => {
                box LxdEngine::create(container, image)
                    .await? as _
            }

            SandboxDef::Shell { dir } => {
                box ShellEngine::create(dir)
                    .await? as _
            }
        };

        Ok(Sandbox::new(engine))
    }
}