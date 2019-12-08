use crate::{LxdClient, LxdContainerName, LxdEventRx, Result};

impl LxdClient {
    pub fn delete(&self, container: &LxdContainerName) -> Result<LxdEventRx> {
        self.invoke(&[
            "delete".to_string(),
            container.to_string(),
            "--force".to_string(),
        ])
    }
}