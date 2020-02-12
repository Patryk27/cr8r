use anyhow::*;

use lib_interop::domain::DExperimentId;

use crate::modules::app::AppContext;

pub struct ExperimentAborter<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ExperimentAborter<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn abort(&mut self, id: DExperimentId) -> Result<()> {
        unimplemented!()
    }
}