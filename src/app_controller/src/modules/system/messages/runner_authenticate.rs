use actix::{Addr, Handler, Message};
use log::*;

use lib_protocol::{RunnerId, RunnerName, RunnerStatus};
use lib_protocol::targets::runner::AuthenticationError;

use crate::modules::{RunnerActor, RunnerEntry, SystemActor};

#[derive(Message)]
#[rtype(result = "Result<(), AuthenticationError>")]
pub struct AuthenticateRunner {
    pub actor: Addr<RunnerActor>,
    pub id: RunnerId,
    pub name: RunnerName,
    pub secret: String,
}

impl Handler<AuthenticateRunner> for SystemActor {
    type Result = Result<(), AuthenticationError>;

    fn handle(&mut self, msg: AuthenticateRunner, _: &mut Self::Context) -> Self::Result {
        if msg.secret != self.secret {
            return Err(AuthenticationError::InvalidSecret);
        }

        if self.runners.contains_key(&msg.id) {
            return Err(AuthenticationError::IdTaken);
        }

        for (_, runner) in &self.runners {
            if &runner.name == &msg.name {
                return Err(AuthenticationError::NameTaken);
            }
        }

        info!("Runner `{}` (`{}`) has been authenticated and parked.", msg.name, msg.id);

        self.runners.insert(msg.id, RunnerEntry {
            id: msg.id,
            actor: msg.actor,
            name: msg.name,
            status: RunnerStatus::Initializing,
        });

        Ok(())
    }
}