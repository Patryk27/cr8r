use crate::{LxdClient, LxdContainerConfig, LxdContainerName, LxdResponseStream, Result};

impl LxdClient {
    pub fn config(&self, container: &LxdContainerName, config: LxdContainerConfig) -> Result<LxdResponseStream> {
        let mut args = vec![
            "config".to_string(),
        ];

        args.extend(config.into_args(container));

        self.invoke(&args)
    }
}