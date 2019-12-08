use crate::{LxdClient, LxdContainerName, LxdEventRx, LxdImageName, Result};

impl LxdClient {
    pub fn launch(&self, image: &LxdImageName, container: &LxdContainerName) -> Result<LxdEventRx> {
        self.invoke(&[
            "launch".to_string(),
            image.to_string(),
            container.to_string(),
            "-csecurity.nesting=true".to_string(),
            "-csecurity.privileged=true".to_string(),
        ])
    }
}