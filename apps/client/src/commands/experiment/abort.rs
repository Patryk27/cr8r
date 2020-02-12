use std::convert::TryInto;

use anyhow::*;

use lib_core_ui::*;
use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentAborter;

pub async fn abort(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    let id = id.try_into().unwrap();

    // @todo
    let _ = spinner! {
        ExperimentAborter::new(ctxt)
            .abort(id)
            .await?
    };

    Ok(())
}