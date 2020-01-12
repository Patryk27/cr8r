use anyhow::Result;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_package(
        &mut self,
        project: String,
        name: String,
        version: String,
    ) -> Result<()> {
        unimplemented!()
    }
}