use crate::{Result, Sandbox};

impl Sandbox {
    // @todo implement timeout
    pub async fn exec(&mut self, cmd: &str) -> Result<()> {
        if let Some(notify) = &self.listener.on_command_started {
            notify(cmd.to_string());
        }

        self.invoke(|lxd| lxd.exec(&self.container, &["bash", "-c", cmd]))
            .await
    }
}