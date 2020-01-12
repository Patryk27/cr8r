use anyhow::Result;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_toolchain(&mut self, project: String, version: String) -> Result<()> {
        unimplemented!()
    }
}