use futures_util::StreamExt;
use log::*;

use crate::backend::{Compiler, System};
use crate::backend::system::SystemRx;

use self::{
    experiments::*,
    runners::*,
};

mod experiments;
mod runners;

pub struct SystemActor {
    rx: SystemRx,
    pub(super) compiler: Compiler,
    pub(super) runners: Runners,
    pub(super) experiments: Experiments,
}

impl SystemActor {
    pub fn new(rx: SystemRx, system: System, compiler: Compiler) -> Self {
        let runners = Runners::new(system.clone());
        let experiments = Experiments::new(system);

        Self { rx, compiler, runners, experiments }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            msg.process(&mut self).await;
        }

        debug!("Actor orphaned, halting");
    }
}