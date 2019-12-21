use crate::contract::CExperiment;
use crate::protocol::core::PAssignment;

#[derive(Clone, Debug)]
pub struct CAssignment {
    pub experiment: CExperiment,
}

impl Into<PAssignment> for CAssignment {
    fn into(self) -> PAssignment {
        PAssignment {
            experiment: Some(
                self.experiment.into(),
            )
        }
    }
}