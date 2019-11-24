use std::process::Command;

use actix::{Actor, Handler, Message};
use log::*;

use crate::modules::TaskExecutorActor;

#[derive(Message)]
pub struct CreateContainer {
    pub name: String,
    pub image: String,
}

impl Handler<CreateContainer> for TaskExecutorActor {
    type Result = ();

    fn handle(&mut self, _: CreateContainer, _: &mut Self::Context) -> Self::Result {
        // Create container
        {
            debug!("Creating container.");

            // @todo delegate to a different package
            Command::new("/snap/bin/lxc")
                .args(&["launch", &self.plan.os, &self.container, "-c", "security.nesting=true", "--ephemeral"])
                .output()
                .unwrap();

            debug!("Container `{}` created.", self.container);
        }

        // Wait for network
        {
            debug!("Waiting for network.");

            // @todo this is a little bit hacky - I feel we could do better
            std::thread::sleep(std::time::Duration::from_secs(5));

            debug!("Network ready.");
        }

        ()
    }
}
