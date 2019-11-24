use actix::{Actor, Addr, SyncContext};
use actix::prelude::Request;
use log::*;

use lib_protocol::{ExecutionStep, ExperimentId};

mod messages;

pub struct TaskExecutorActor;

impl TaskExecutorActor {
    pub fn new() -> Self {
        Self
    }

    pub fn create_container(addr: &Addr<Self>, name: String, image: String) -> Request<Self, messages::CreateContainer> {
        addr.send(messages::CreateContainer { name, image })
    }
}

impl Actor for TaskExecutorActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Actor started.");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("Actor stopped.");
    }
}