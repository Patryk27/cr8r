use crate::{LxdClient, LxdContainerName, LxdResponseStream, Result};

impl LxdClient {
    pub fn delete(&self, container: &LxdContainerName) -> Result<LxdResponseStream> {
        self.invoke(&[
            "delete".to_string(),
            container.to_string(),
            "--force".to_string(),
        ])
    }
}