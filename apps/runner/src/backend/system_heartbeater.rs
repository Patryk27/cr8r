use crate::core::SessionClient;

pub use self::actor::*;

mod actor;

pub struct SystemHeartbeater;

impl SystemHeartbeater {
    pub fn spawn(client: SessionClient) {
        tokio::spawn(SystemHeartbeatActor::new(
            client,
        ).start());
    }
}