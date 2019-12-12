use crate::{LxdClient, LxdContainerName, LxdResponseStream, Result};

impl LxdClient {
    pub fn exec(&self, container: &LxdContainerName, args: &[&str]) -> Result<LxdResponseStream> {
        let mut invoke_args = vec![
            "exec".to_string(),
            container.to_string(),
            "--".to_string(),
        ];

        for arg in args {
            invoke_args.push(arg.to_string());
        }

        self.invoke(&invoke_args)
    }
}