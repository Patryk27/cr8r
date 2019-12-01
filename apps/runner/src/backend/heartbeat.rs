use crate::core::Session;

pub use self::actor::*;

mod actor;

pub struct Heartbeat;

impl Heartbeat {
    pub fn spawn(mut session: Session) {
        tokio::spawn(HeartbeatActor::new(
            session,
        ).start());
    }
}