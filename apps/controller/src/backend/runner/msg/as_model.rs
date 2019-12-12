use lib_protocol::core::p_runner::p_status::*;
use lib_protocol::core::p_runner::PStatus;
use lib_protocol::core::PRunner;

use crate::backend::runner::RunnerActor;

pub fn process(actor: &mut RunnerActor) -> PRunner {
    // @todo provide actual status
    let status = Op::Idle(PIdle {
        since: actor.joined_at.to_rfc3339(),
    });

    PRunner {
        id: actor.id.clone(),
        name: actor.name.clone(),
        joined_at: actor.joined_at.to_rfc3339(),
        heartbeaten_at: actor.heartbeaten_at.to_rfc3339(),

        status: Some(PStatus {
            op: Some(status),
        }),
    }
}