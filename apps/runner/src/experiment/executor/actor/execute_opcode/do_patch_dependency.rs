use anyhow::Result;

use lib_interop::domain::DAttachmentId;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_patch_dependency(
        &mut self,
        project: String,
        dep_registry: String,
        dep_name: String,
        dep_source_attachment_id: DAttachmentId,
    ) -> Result<()> {
        unimplemented!()
    }
}