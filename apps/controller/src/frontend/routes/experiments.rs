use actix_web::{Responder, web};
use futures::{Future, IntoFuture};

use lib_client_protocol::{CreateExperimentRequest, CreateExperimentResponse};

use crate::frontend::State;

pub fn launch(
    state: web::Data<State>,
    request: web::Json<CreateExperimentRequest>,
) -> impl IntoFuture<Item=impl Responder, Error=()> {
    let experiment = request.into_inner().experiment;

    state.system.launch_experiment(experiment)
        .map(|experiment_id| CreateExperimentResponse { experiment_id })
        .map(web::Json)
        .map_err(|_| ())
}

pub fn get() -> impl Responder {
    // @todo
}

pub fn abort() -> impl Responder {
    // @todo
}