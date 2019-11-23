use actix_web::{HttpRequest, Responder, web};
use actix_web_actors::ws;

use crate::http::HttpState;
use crate::modules::{Runner, RunnerId};

pub fn accept_runner(req: HttpRequest, state: web::Data<HttpState>, stream: web::Payload) -> impl Responder {
    let runner = Runner::new(
        RunnerId::new_v4(),
        state.system.clone(),
    );

    ws::start(runner, &req, stream)
        .unwrap()
}