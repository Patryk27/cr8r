use crate::{Result, Sandbox};

impl Sandbox {
    pub async fn destroy(&mut self) -> Result<()> {
        self.invoke(|lxd| lxd.delete(&self.container))
            .await
    }
}