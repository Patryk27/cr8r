use std::collections::HashMap;

use actix::{Actor, Addr, Context};
use actix::prelude::Request;
use log::*;

use crate::modules::{ExperimentEntry, ExperimentId, Runner, RunnerEntry, RunnerId, RunnerName};

mod messages;

pub struct System {
    secret: String,
    runners: HashMap<RunnerId, RunnerEntry>,
    experiments: HashMap<ExperimentId, ExperimentEntry>,
}

impl System {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            runners: HashMap::new(),
            experiments: HashMap::new(),
        }
    }

    pub fn authenticate_runner(
        addr: &Addr<Self>,
        actor: Addr<Runner>,
        id: RunnerId,
        name: RunnerName,
        secret: String,
    ) -> Request<Self, messages::AuthenticateRunner> {
        addr.send(messages::AuthenticateRunner { actor, id, name, secret })
    }

    pub fn deauthenticate_runner(
        addr: &Addr<Self>,
        id: RunnerId,
    ) -> Request<Self, messages::DeauthenticateRunner> {
        addr.send(messages::DeauthenticateRunner { id })
    }

    pub fn get_status(addr: &Addr<Self>) -> Request<Self, messages::GetControllerStatus> {
        addr.send(messages::GetControllerStatus)
    }
}

impl Actor for System {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Actor started.");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("Actor stopped.");
    }
}