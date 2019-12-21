use lib_interop::contract::CRunner;

use crate::backend::runner::RunnerActor;

pub fn get_model(actor: &mut RunnerActor) -> CRunner {
    unimplemented!()

//    // @todo provide actual status
//    let status = Op::Idle(PIdle {
//        since: actor.joined_at.to_rfc3339(),
//    });
//
//    PRunner {
//        id: actor.id.clone(),
//        name: actor.name.clone(),
//        joined_at: actor.joined_at.to_rfc3339(),
//        last_heartbeat_at: actor.last_heartbeat_at.to_rfc3339(),
//        status,
//    }
}