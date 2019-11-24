use actix::{Handler, Message, MessageResult};
use log::*;

use lib_protocol::{Experiment, ExperimentDefinition, ExperimentId, ExperimentStatus};

use crate::modules::SystemActor;

#[derive(Message)]
#[rtype(result = "ExperimentId")]
pub struct CreateExperiment {
    pub definition: ExperimentDefinition,
}

impl Handler<CreateExperiment> for SystemActor {
    type Result = MessageResult<CreateExperiment>;

    fn handle(&mut self, msg: CreateExperiment, ctx: &mut Self::Context) -> Self::Result {
        let id = ExperimentId::new_v4();

        self.experiments.insert(id, Experiment {
            id,
            execution_plans: self.compiler.compile(&msg.definition),
            status: ExperimentStatus::AwaitingRunner,
            definition: msg.definition,
        });

        self.awaiting_experiments.push_back(id);

        info!("Experiment `{}` has been created and enqueued for running.", id);

        self.process_waiting_experiments(ctx);

        MessageResult(id)
    }
}