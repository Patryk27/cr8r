use actix_web::{HttpRequest, Responder, web};
use actix_web_actors::ws;

use lib_protocol::RunnerId;

use crate::http::State;
use crate::modules::RunnerActor;

pub fn accept_runner(state: web::Data<State>, request: HttpRequest, stream: web::Payload) -> impl Responder {
    let runner = RunnerActor::new(
        RunnerId::new_v4(),
        state.system.clone(),
    );

    ws::start(runner, &request, stream)
        .unwrap()
}