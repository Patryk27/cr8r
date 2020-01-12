use anyhow::Result;

use lib_interop::domain::DAttachmentId;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_patch_package(
        &mut self,
        project: String,
        pkg_name: String,
        pkg_attachment_id: DAttachmentId,
    ) -> Result<()> {
        unimplemented!()
    }
}