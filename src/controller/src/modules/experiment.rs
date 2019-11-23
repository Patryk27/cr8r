use actix::{Actor, Addr, Context};

use crate::modules::System;

pub use self::{
    entry::*,
    id::*,
};

mod entry;
mod id;

pub struct Experiment {
    id: ExperimentId,
    system: Addr<System>,
}

impl Experiment {
    pub fn new(id: ExperimentId, system: Addr<System>) -> Self {
        Self { id, system }
    }
}

impl Actor for Experiment {
    type Context = Context<Self>;
}