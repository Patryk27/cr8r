use actix_web::{Responder, web};
use futures::{Future, IntoFuture};

use lib_protocol::targets::client::GetControllerStatusResponse;

use crate::http::State;
use crate::modules::SystemActor;

pub fn get_controller_status(state: web::Data<State>) -> impl IntoFuture<Item=impl Responder, Error=()> {
    SystemActor::get_status(&state.system)
        .map(|status| GetControllerStatusResponse { status })
        .map(web::Json)
        .map_err(|_| ())
}