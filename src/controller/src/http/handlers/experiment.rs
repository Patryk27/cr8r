use actix_web::{Responder, web};

use lib_protocol as proto;

pub fn create_experiment(experiment: web::Json<proto::ExperimentDefinition>) -> impl Responder {
    // @todo
}

pub fn get_experiment() -> impl Responder {
    // @todo
}

pub fn delete_experiment() -> impl Responder {
    // @todo
}