use actix::{Handler, Message};
use log::*;

use lib_protocol_core::RunnerId;

use crate::backend::SystemActor;

#[derive(Message)]
pub struct DeauthenticateRunner {
    pub id: RunnerId,
}

impl Handler<DeauthenticateRunner> for SystemActor {
    type Result = ();

    fn handle(&mut self, msg: DeauthenticateRunner, _: &mut Self::Context) -> Self::Result {
        // @todo what should happen to associated experiment?

        if let Some(runner) = self.runners.remove(&msg.id) {
            // @todo remove from idle runners too

            info!("Runner `{}` (`{}`) has been de-authenticated.", runner.name, msg.id);
        } else {
            error!("Tried to de-authenticate a non-existing runner `{}`.", msg.id);
        }
    }
}