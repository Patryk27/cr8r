use crate::core::SessionClient;

pub(self) use self::actor::*;

mod actor;

pub struct SystemHeartbeater;

impl SystemHeartbeater {
    pub fn new(client: SessionClient) {
        tokio::spawn(SystemHeartbeaterActor::new(
            client,
        ).main());
    }
}