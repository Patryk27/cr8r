use std::convert::TryFrom;

use crate::{Error, Result};
use crate::protocol::core::PExperimentEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CExperimentEvent {
    //
}

impl TryFrom<PExperimentEvent> for CExperimentEvent {
    type Error = Error;

    fn try_from(event: PExperimentEvent) -> Result<Self> {
        unimplemented!()
    }
}