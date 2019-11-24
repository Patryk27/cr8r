use actix::{Actor, ActorContext, ActorFuture, Addr, AsyncContext, StreamHandler, WrapFuture};
use actix::dev::Future;
use actix::prelude::Request;
use actix_web_actors::ws;
use log::*;

use lib_protocol::{ExecutionPlan, ExperimentId, RunnerId};
use lib_protocol::targets::runner::{ControllerMessage, RunnerMessage};

use crate::modules::SystemActor;

pub use self::entry::*;

mod entry;
mod messages;

pub struct RunnerActor {
    id: RunnerId,
    system: Addr<SystemActor>,
    authenticated: bool,
}

impl RunnerActor {
    pub fn new(id: RunnerId, system: Addr<SystemActor>) -> Self {
        Self {
            id,
            system,
            authenticated: false,
        }
    }

    pub fn launch_experiment(
        addr: &Addr<Self>,
        id: ExperimentId,
        plans: Vec<ExecutionPlan>,
    ) -> Request<Self, messages::LaunchExperiment> {
        addr.send(messages::LaunchExperiment {
            id,
            plans,
        })
    }

    fn process_request(&mut self, msg: ControllerMessage, ctx: &mut ws::WebsocketContext<Self>) {
        debug!("Processing request: {:?}", msg);

        match msg {
            ControllerMessage::Authenticate { name, secret } => {
                let fut = SystemActor::authenticate_runner(&self.system, ctx.address(), self.id, name, secret)
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
                    version: lib_protocol::version(),
                });
            }

            ControllerMessage::Report { report } => {
                // @todo
                debug!("{:?}", report);
            }

            ControllerMessage::Unpark => {
                let fut = SystemActor::unpark_runner(&self.system, self.id)
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
                SystemActor::deauthenticate_runner(&self.system, self.id)
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
                self.process_request(
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