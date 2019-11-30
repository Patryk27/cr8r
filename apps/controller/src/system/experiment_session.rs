use std::result;

use bastion::prelude::ChildRef;
use uuid::Uuid;

use crate::system::System;

pub type ExperimentId = Uuid;

#[derive(Clone)]
pub struct ExperimentSession {
    conn: ChildRef,
}

impl ExperimentSession {
    pub async fn start(system: System, id: ExperimentId) -> result::Result<(), ()> {
        unimplemented!()
    }

    pub fn heartbeat(&self) {
        unimplemented!()
    }
}