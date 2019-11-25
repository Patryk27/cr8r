use actix_web::{HttpRequest, Responder, web};
use actix_web_actors::ws;

use crate::backend::Runner;
use crate::frontend::State;

pub fn accept(state: web::Data<State>, request: HttpRequest, stream: web::Payload) -> impl Responder {
    let runner = Runner::create(
        state.system.clone(),
    );

    ws::start(runner, &request, stream)
        .unwrap()
}