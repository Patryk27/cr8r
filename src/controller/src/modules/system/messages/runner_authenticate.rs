use actix::{Addr, Handler, Message};
use log::*;

use lib_protocol as proto;

use crate::modules::{Runner, RunnerEntry, RunnerId, RunnerName, System};

#[derive(Message)]
#[rtype(result = "Result<(), proto::runner::AuthenticationError>")]
pub struct AuthenticateRunner {
    pub actor: Addr<Runner>,
    pub id: RunnerId,
    pub name: RunnerName,
    pub secret: String,
}

impl Handler<AuthenticateRunner> for System {
    type Result = Result<(), proto::runner::AuthenticationError>;

    fn handle(&mut self, msg: AuthenticateRunner, ctx: &mut Self::Context) -> Self::Result {
        if msg.secret != self.secret {
            return Err(proto::runner::AuthenticationError::InvalidSecret);
        }

        if self.runners.contains_key(&msg.id) {
            return Err(proto::runner::AuthenticationError::IdTaken);
        }

        for (_, runner) in &self.runners {
            if &runner.name == &msg.name {
                return Err(proto::runner::AuthenticationError::NameTaken);
            }
        }

        info!("Runner `{}` (`{}`) has been authenticated.", msg.name, msg.id);

        self.runners.insert(msg.id, RunnerEntry {
            actor: msg.actor,
            name: msg.name,
            status: proto::RunnerStatus::Idle,
        });

        Ok(())
    }
}