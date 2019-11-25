use actix::{Handler, Message, MessageResult};
use log::*;

use lib_protocol_core::{Experiment, ExperimentDefinition, ExperimentId, ExperimentStatus};

use crate::backend::SystemActor;

#[derive(Message)]
#[rtype(result = "ExperimentId")]
pub struct CreateExperiment {
    pub definition: ExperimentDefinition,
}

impl Handler<CreateExperiment> for SystemActor {
    type Result = MessageResult<CreateExperiment>;

    fn handle(&mut self, msg: CreateExperiment, ctx: &mut Self::Context) -> Self::Result {
        let id = ExperimentId::new_v4();

        let scenarios = self.compiler.compile(&msg.definition);

        self.experiments.insert(id, Experiment {
            id,
            scenarios,
            status: ExperimentStatus::AwaitingRunner,
            definition: msg.definition,
        });

        self.awaiting_experiments.push_back(id);

        info!("Experiment `{}` has been created and enqueued for running.", id);

        self.process_waiting_experiments(ctx);

        MessageResult(id)
    }
}