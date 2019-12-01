use crate::core::SessionClient;

pub use self::actor::*;

mod actor;

pub struct Heartbeat;

impl Heartbeat {
    pub fn spawn(session_client: SessionClient) {
        tokio::spawn(HeartbeatActor::new(
            session_client,
        ).start());
    }
}