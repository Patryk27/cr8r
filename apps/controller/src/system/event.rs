use std::sync::Arc;

use lib_interop::models::{DAttachmentId, DExperimentId, DRunnerId, DRunnerName};

#[derive(Clone, Debug)]
pub enum SystemEvent {
    AttachmentCreated {
        id: DAttachmentId,
    },

    AttachmentDeleted {
        id: DAttachmentId,
    },

    ExperimentCreated {
        id: DExperimentId,
    },

    ExperimentDeleted {
        id: DExperimentId,
    },

    RunnerJoined {
        id: DRunnerId,
        name: Arc<DRunnerName>,
    },

    RunnerLeft {
        id: DRunnerId,
    },

    RunnerTurnedZombie {
        id: DRunnerId,
    },

    RunnerTurnedAlive {
        id: DRunnerId,
    },
}
