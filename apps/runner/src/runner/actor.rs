use std::process::exit;

use actix::{Actor, Context, StreamHandler, System};
use actix::io::WriteHandler;
use actix_web_actors::ws;
use log::*;

use lib_runner_protocol::{ControllerMessage, RunnerMessage};

use crate::{RunnerSocket, RunnerState};

pub struct RunnerActor {
    name: String,
    secret: String,
    socket: RunnerSocket,
    state: RunnerState,
}

impl RunnerActor {
    pub fn new(name: String, secret: String, socket: RunnerSocket) -> Self {
        Self {
            name,
            secret,
            socket,
            state: RunnerState::Initializing,
        }
    }

    fn process_response(&mut self, msg: RunnerMessage, ctx: &mut Context<Self>) {
        debug!("Processing response: {:?}", msg);

        match msg {
            RunnerMessage::Authenticate { result } => {
                match result {
                    Ok(()) => {
                        info!("Authenticated successfully.");

                        self.state = RunnerState::Idle;
                        self.socket.send(ControllerMessage::Unpark);
                    }

                    Err(err) => {
                        error!("Controller rejected our authentication request: {:?}", err);
                        error!("Shutting down.");

                        exit(2);
                    }
                }
            }

            RunnerMessage::Hello { version } => {
                if !lib_protocol_core::is_compatible_with(&version) {
                    error!("Controller is not compatible with us (protocol version mismatch - controller uses `{}`, we use `{}`)", version, lib_protocol_core::version());
                    error!("Shutting down.");

                    exit(2);
                }

                self.socket.send(ControllerMessage::Authenticate {
                    name: self.name.clone(),
                    secret: self.secret.clone(),
                });
            }

            RunnerMessage::LaunchExperiment { id, scenarios } => {
                if self.state != RunnerState::Idle {
                    // @todo
                    panic!("Runner is not ready to launch experiments yet.");
                }

//                let container_name = format!("cr8r-{}", id);
//
//                let task_executor = SyncArbiter::start(1, move || {
//                    TaskExecutorActor::new()
//                });
//
//                let plan_executor = PlanExecutorActor::new(plan, task_executor.clone())
//                    .start();

                self.state = RunnerState::RunningExperiment {
                    // @todo
                };
            }
        }
    }
}

impl Actor for RunnerActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Actor started.");

        info!("Connection acquired, negotiating protocols.");
        self.socket.send(ControllerMessage::Hello);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Actor stopped.");

        // Since this actor is the main one, stopping it means that we have to shut down the entire system too
        // (otherwise the application would just got stuck waiting for nothing more to happen).
        System::current().stop();
    }
}

impl StreamHandler<ws::Frame, ws::ProtocolError> for RunnerActor {
    fn handle(&mut self, msg: ws::Frame, ctx: &mut Self::Context) {
        debug!("Received packet: {:?}", msg);

        if let Some(msg) = self.socket.parse(msg) {
            self.process_response(msg, ctx);
        }
    }
}

impl WriteHandler<ws::ProtocolError> for RunnerActor {
    //
}