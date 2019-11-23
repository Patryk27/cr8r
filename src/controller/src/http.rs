use std::error::Error;

use actix::Addr;
use actix_web::{App, HttpServer as ActixHttpServer, web};
use actix_web::middleware::Logger;
use snafu::ResultExt;

use crate::modules::System;

pub use self::state::*;

mod handlers;
mod state;

pub struct HttpServer<'a> {
    bind: &'a str,
    system: Addr<System>,
}

impl<'a> HttpServer<'a> {
    pub fn new(bind: &'a str, system: Addr<System>) -> Self {
        Self { bind, system }
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        use crate::error;
        use handlers::*;

        let state = HttpState {
            system: self.system,
        };

        let server = ActixHttpServer::new(move || {
            let controller = web::scope("/controller")
                .route("/status", web::get().to_async(get_controller_status));

            let experiments = web::scope("/experiments")
                .route("/", web::post().to(create_experiment))
                .route("/{id}", web::get().to(get_experiment))
                .route("/{id}", web::delete().to(delete_experiment));

            let runners = web::scope("/runners")
                .route("/ws", web::get().to(accept_runner));

            App::new()
                .wrap(Logger::default())
                .data(state.clone())
                .service(controller)
                .service(experiments)
                .service(runners)
        });

        server
            .bind(self.bind)
            .context(error::FailedToStart)?
            .start();

        Ok(())
    }
}