use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use snafu::ResultExt;

use crate::{error, Result};
use crate::backend::System;

use self::routes::*;
pub use self::state::*;

mod routes;
mod state;

pub fn start(bind: String, system: System) -> Result<()> {
    let state = State {
        system,
    };

    let server = HttpServer::new(move || {
        let controller = web::scope("/controller")
            .route("/status", web::get().to_async(controller::status));

        let experiments = web::scope("/experiments")
            .route("", web::post().to_async(experiments::launch))
            .route("/{id}", web::get().to(experiments::get))
            .route("/{id}", web::delete().to(experiments::abort));

        let runners = web::scope("/runners")
            .route("/ws", web::get().to(runners::accept));

        App::new()
            .wrap(Logger::default())
            .data(state.clone())
            .service(controller)
            .service(experiments)
            .service(runners)
    });

    server
        .bind(bind)
        .context(error::FailedToStart)?
        .start();

    Ok(())
}