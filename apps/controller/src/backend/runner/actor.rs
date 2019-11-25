use actix::{Actor, ActorContext, ActorFuture, AsyncContext, StreamHandler, WrapFuture};
use actix::dev::Future;
use actix_web_actors::ws;
use log::*;

use lib_protocol_core::RunnerId;
use lib_runner_protocol::{ControllerMessage, RunnerMessage};

use crate::backend::System;

pub struct RunnerActor {
    pub id: RunnerId,
    pub system: System,
    pub authenticated: bool,
}

impl RunnerActor {
    fn process(&mut self, msg: ControllerMessage, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Processing message: {:?}", msg);

        match msg {
            ControllerMessage::Authenticate { name, secret } => {
                let fut = self.system.authenticate_runner(ctx.address().into(), self.id, name, secret)
                    .map_err(|_| ())
                    .into_actor(self)
                    .map(|result, this, ctx| {
                        if result.is_ok() {
                            this.authenticated = true;
                        }

                        ctx.text(RunnerMessage::Authenticate { result });
                    });

                ctx.spawn(fut);
            }

            ControllerMessage::Hello => {
                ctx.text(RunnerMessage::Hello {
                    version: lib_protocol_core::version(),
                });
            }

            ControllerMessage::Report { report } => {
                // @todo
                debug!("{:?}", report);
            }

            ControllerMessage::Unpark => {
                let fut = self.system.unpark_runner(self.id)
                    .map_err(|_| ())
                    .into_actor(self);

                ctx.spawn(fut);
            }
        }
    }
}

impl Actor for RunnerActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Actor started.");
        debug!("-> id: {:?}", self.id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        if self.authenticated {
            debug!("De-authenticating runner from the system.");

            ctx.wait(
                self.system.deauthenticate_runner(self.id)
                    .map_err(|_| ())
                    .into_actor(self)
            );
        }

        debug!("Actor stopped.");
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for RunnerActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }

            ws::Message::Text(msg) => {
                self.process(
                    ControllerMessage::unmarshal(msg),
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

            ws::Message::Nop | ws::Message::Pong(_) => {
                //
            }
        }
    }
}