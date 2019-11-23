use actix::{Actor, ActorContext, ActorFuture, Addr, AsyncContext, StreamHandler, WrapFuture};
use actix::dev::Future;
use actix_web_actors::ws;
use log::*;

use lib_protocol as proto;

use crate::modules::System;

pub use self::{
    entry::*,
    id::*,
    name::*,
};

mod entry;
mod id;
mod name;

pub struct Runner {
    id: RunnerId,
    system: Addr<System>,
    authenticated: bool,
}

impl Runner {
    pub fn new(id: RunnerId, system: Addr<System>) -> Self {
        Self {
            id,
            system,
            authenticated: false,
        }
    }

    fn process_request(&mut self, msg: proto::runner::Request, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Processing request: {:?}", msg);

        match msg {
            proto::runner::Request::Authenticate { name, secret } => {
                let f = System::authenticate_runner(&self.system, ctx.address(), self.id, name, secret)
                    .map_err(|_| ())
                    .into_actor(self)
                    .map(|result, this, ctx| {
                        if result.is_ok() {
                            this.authenticated = true;
                        }

                        ctx.text(proto::runner::Response::Authenticate { result });
                    });

                ctx.spawn(f);
            }

            proto::runner::Request::Hello => {
                ctx.text(proto::runner::Response::Hello {
                    version: lib_protocol::version(),
                });
            }
        }
    }
}

impl Actor for Runner {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Actor started.");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("Actor stopped.");

        if self.authenticated {
            ctx.wait(
                System::deauthenticate_runner(&self.system, self.id)
                    .map_err(|_| ())
                    .into_actor(self)
            );
        }
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Runner {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }

            ws::Message::Pong(_) => {
                // @todo
            }

            ws::Message::Text(msg) => {
                self.process_request(
                    proto::runner::Request::unmarshal(msg),
                    ctx,
                );
            }

            ws::Message::Binary(msg) => {
                error!("We've received a binary message - not sure what to do.");
                error!("Message says: {:?}", msg);
            }

            ws::Message::Close(_) => {
                ctx.stop();
            }

            ws::Message::Nop => {
                //
            }
        }
    }
}