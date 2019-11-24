use std::error::Error;

use actix::Addr;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use snafu::ResultExt;

use crate::modules::SystemActor;

pub use self::state::*;

mod handlers;
mod state;

pub struct Server<'a> {
    bind: &'a str,
    system: Addr<SystemActor>,
}

impl<'a> Server<'a> {
    pub fn new(bind: &'a str, system: Addr<SystemActor>) -> Self {
        Self { bind, system }
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        use crate::error;
        use handlers::*;

        let state = State {
            system: self.system,
        };

        let server = HttpServer::new(move || {
            let controller = web::scope("/controller")
                .route("/status", web::get().to_async(get_controller_status));

            let experiments = web::scope("/experiments")
                .route("", web::post().to_async(create_experiment))
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