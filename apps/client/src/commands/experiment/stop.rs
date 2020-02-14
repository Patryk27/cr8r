use anyhow::*;

use lib_core_ui::*;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentStopper;

pub async fn stop(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    // @todo
    let _ = spinner! {
        ExperimentStopper::new(ctxt)
            .stop(id)
            .await?
    };

    MessageWidget::info("Success:", "Experiment has been stopped")
        .println();

    Ok(())
}