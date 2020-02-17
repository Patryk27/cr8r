use anyhow::*;

use lib_core_ui::*;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentRepository;

pub async fn delete(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    let _ = spinner! {
        ExperimentRepository::new(ctxt)
            .await?
            .delete(id)
            .await?
    };

    MessageWidget::info("Success:", "Experiment has been deleted")
        .println();

    Ok(())
}