use anyhow::*;

use lib_core_ui::*;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;

pub async fn delete(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    let _ = spinner! {
        ctxt.conn()
            .await?
            .experiments()
            .delete(id.into())
            .await?
    };

    MessageWidget::info("Success:", "Experiment has been deleted")
        .println();

    Ok(())
}