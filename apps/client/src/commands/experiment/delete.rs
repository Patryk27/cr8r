use anyhow::*;

use lib_core_ui::*;
use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentDeleter;

pub async fn delete(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    // @todo
    let _ = spinner! {
        ExperimentDeleter::new(ctxt)
            .delete(id)
            .await?
    };

    MessageWidget::info("Succes:", "Experiment has been deleted")
        .println();

    Ok(())
}