use anyhow::*;

use lib_interop::proto::core::PExperimentId;

use crate::app::AppContext;

pub async fn abort(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    unimplemented!()
}