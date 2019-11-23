use actix_web::{Responder, web};
use futures::{Future, IntoFuture};

use lib_protocol as proto;

use crate::http::HttpState;
use crate::modules::System;

pub fn get_controller_status(state: web::Data<HttpState>) -> impl IntoFuture<Item=impl Responder, Error=()> {
    System::get_status(&state.system)
        .map(|status| proto::client::GetControllerStatusResponse { status })
        .map(web::Json)
        .map_err(|_| ())
}