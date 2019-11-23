use actix::{Handler, Message};
use log::*;

use crate::modules::{RunnerId, System};

#[derive(Message)]
pub struct DeauthenticateRunner {
    pub id: RunnerId,
}

impl Handler<DeauthenticateRunner> for System {
    type Result = ();

    fn handle(&mut self, msg: DeauthenticateRunner, ctx: &mut Self::Context) -> Self::Result {
        if let Some(runner) = self.runners.remove(&msg.id) {
            info!("Runner `{}` (`{}`) has been de-authenticated.", runner.name, msg.id);
        } else {
            error!("Tried to de-authenticate a non-existing runner `{}`.", msg.id);
        }
    }
}