use std::result;

use bastion::prelude::ChildRef;
use uuid::Uuid;

use crate::system::System;

pub type RunnerId = Uuid;
pub type RunnerName = String;
pub type RunnerSecret = String;

mod actor;

#[derive(Clone, Debug)]
pub struct RunnerSession {
    conn: ChildRef,
}

impl RunnerSession {
    pub async fn start(system: System, id: RunnerId, name: RunnerName) -> result::Result<(), ()> {
        unimplemented!()
    }

    pub fn heartbeat(&self) {
        unimplemented!()
    }
}