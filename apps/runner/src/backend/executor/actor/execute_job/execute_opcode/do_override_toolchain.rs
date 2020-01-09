use anyhow::Result;

use crate::backend::executor::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_override_toolchain(&mut self, project: String, version: String) -> Result<()> {
        unimplemented!()
    }
}