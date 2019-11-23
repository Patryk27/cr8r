use actix::{Actor, ActorContext, Context, StreamHandler, System};
use actix::io::WriteHandler;
use actix_web_actors::ws;
use log::*;

use lib_protocol as proto;

pub use self::socket::*;

mod socket;

pub struct Runner {
    name: String,
    secret: String,
    socket: RunnerSocket,
}

impl Runner {
    pub fn new(name: String, secret: String, socket: RunnerSocket) -> Self {
        Self { name, secret, socket }
    }

    fn process_response(&mut self, msg: proto::runner::Response, ctx: &mut Context<Self>) {
        debug!("Processing response: {:?}", msg);

        match msg {
            proto::runner::Response::Authenticate { result } => {
                match result {
                    Ok(()) => {
                        info!("Authenticated successfully.");
                        info!("Runner is ready to accept commands.");
                    }

                    Err(err) => {
                        error!("Controller rejected our authentication request: {:?}", err);
                        error!("Stopping runner.");

                        ctx.stop();
                    }
                }
            }

            proto::runner::Response::Hello { version } => {
                if !lib_protocol::is_compatible_with(&version) {
                    error!("Controller is not compatible with us (protocol version mismatch - controller uses `{}`, we use `{}`)", version, lib_protocol::version());
                    error!("Stopping runner.");

                    // @todo we could provide a flag to override this behavior
                    ctx.stop();
                    return;
                }

                self.socket.send(proto::runner::Request::Authenticate {
                    name: self.name.clone(),
                    secret: self.secret.clone(),
                });
            }
        }
    }
}

impl Actor for Runner {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Actor started.");

        self.socket.send(proto::runner::Request::Hello);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Actor stopped.");

        System::current().stop();
    }
}

impl StreamHandler<ws::Frame, ws::ProtocolError> for Runner {
    fn handle(&mut self, msg: ws::Frame, ctx: &mut Self::Context) {
        debug!("Received packet: {:?}", msg);

        if let Some(msg) = self.socket.recv(msg) {
            self.process_response(msg, ctx);
        }
    }
}

impl WriteHandler<ws::ProtocolError> for Runner {
    //
}