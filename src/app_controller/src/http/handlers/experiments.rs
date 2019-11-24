use actix_web::{Responder, web};
use futures::{Future, IntoFuture};

use lib_protocol::targets::client::{CreateExperimentRequest, CreateExperimentResponse};

use crate::http::State;
use crate::modules::SystemActor;

pub fn create_experiment(state: web::Data<State>, request: web::Json<CreateExperimentRequest>) -> impl IntoFuture<Item=impl Responder, Error=()> {
    SystemActor::create_experiment(&state.system, request.into_inner().experiment)
        .map(|experiment_id| CreateExperimentResponse { experiment_id })
        .map(web::Json)
        .map_err(|_| ())
}

pub fn get_experiment() -> impl Responder {
    // @todo
}

pub fn delete_experiment() -> impl Responder {
    // @todo
}