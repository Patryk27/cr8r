use actix::Addr;

use crate::modules::SystemActor;

#[derive(Clone)]
pub struct State {
    pub system: Addr<SystemActor>,
}