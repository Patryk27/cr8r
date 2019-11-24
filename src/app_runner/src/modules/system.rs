use actix::{Actor, ActorContext, Addr, Context, StreamHandler, SyncArbiter, System};
use actix::io::WriteHandler;
use actix_web_actors::ws;
use log::*;

use lib_protocol::targets::runner::{ControllerMessage, RunnerMessage};

use crate::modules::{PlanExecutorActor, TaskExecutorActor};

pub use self::socket::*;

mod socket;

pub struct SystemActor {
    name: String,
    secret: String,
    socket: RunnerSocket,
    state: SystemState,
}

#[derive(Eq, PartialEq)]
enum SystemState {
    Idle,

    Initializing,

    RunningExperiment {
        task_executor: Addr<TaskExecutorActor>,
        plan_executor: Addr<PlanExecutorActor>,
    },
}

impl SystemActor {
    pub fn new(name: String, secret: String, socket: RunnerSocket) -> Self {
        Self {
            name,
            secret,
            socket,
            state: SystemState::Initializing,
        }
    }

    fn process_response(&mut self, msg: RunnerMessage, ctx: &mut Context<Self>) {
        debug!("Processing response: {:?}", msg);

        match msg {
            RunnerMessage::Authenticate { result } => {
                match result {
                    Ok(()) => {
                        info!("Authenticated successfully.");

                        self.state = SystemState::Idle;
                        self.socket.send(ControllerMessage::Unpark);
                    }

                    Err(err) => {
                        error!("Controller rejected our authentication request: {:?}", err);
                        error!("Shutting down.");

                        ctx.stop();
                    }
                }
            }

            RunnerMessage::Hello { version } => {
                if !lib_protocol::is_compatible_with(&version) {
                    error!("Controller is not compatible with us (protocol version mismatch - controller uses `{}`, we use `{}`)", version, lib_protocol::version());
                    error!("Shutting down.");

                    // @todo we could provide a flag to override this behavior
                    ctx.stop();

                    return;
                }

                self.socket.send(ControllerMessage::Authenticate {
                    name: self.name.clone(),
                    secret: self.secret.clone(),
                });
            }

            RunnerMessage::LaunchExperiment { id, plans } => {
                if self.state != SystemState::Idle {
                    // @todo
                    panic!("Runner is not ready to launch experiments yet.");
                }

                let container_name = format!("cr8r-{}", id);

                let task_executor = SyncArbiter::start(1, move || {
                    TaskExecutorActor::new()
                });

                let plan_executor = PlanExecutorActor::new(plan, task_executor.clone())
                    .start();

                self.state = SystemState::RunningExperiment {
                    task_executor,
                    plan_executor,
                };
            }
        }
    }
}

impl Actor for SystemActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Actor started.");

        self.socket.send(ControllerMessage::Hello);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Actor stopped.");

        // Since this actor is the main one, stopping it means that we have to shut down the entire system too
        // (otherwise the application would just got stuck waiting for nothing more to happen).
        System::current().stop();
    }
}

impl StreamHandler<ws::Frame, ws::ProtocolError> for SystemActor {
    fn handle(&mut self, msg: ws::Frame, ctx: &mut Self::Context) {
        debug!("Received packet: {:?}", msg);

        if let Some(msg) = self.socket.recv(msg) {
            self.process_response(msg, ctx);
        }
    }
}

impl WriteHandler<ws::ProtocolError> for SystemActor {
    //
}