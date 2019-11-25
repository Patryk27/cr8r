use actix_web::{Responder, web};
use futures::{Future, IntoFuture};

use lib_client_protocol::GetControllerStatusResponse;

use crate::frontend::State;

pub fn status(state: web::Data<State>) -> impl IntoFuture<Item=impl Responder, Error=()> {
    state.system.status()
        .map(|status| GetControllerStatusResponse { status })
        .map(web::Json)
        .map_err(|_| ())
}