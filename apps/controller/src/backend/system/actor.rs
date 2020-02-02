use log::*;
use tokio::stream::StreamExt;

use crate::backend::Compiler;
use crate::backend::system::SystemRx;

pub use self::{
    experiments::*,
    runners::*,
};

mod experiments;
mod runners;

pub struct SystemActor {
    pub compiler: Compiler,
    pub runners: Runners,
    pub experiments: Experiments,
}

impl SystemActor {
    pub async fn start(mut self, mut rx: SystemRx) {
        debug!("Actor started");

        while let Some(msg) = rx.next().await {
            msg.handle(&mut self)
                .await;
        }

        debug!("Actor orphaned, halting");
    }
}
