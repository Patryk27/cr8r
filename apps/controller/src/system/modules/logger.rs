use log::*;
use tokio::task::spawn;

use crate::system::SystemEventBus;

pub struct Logger;

impl Logger {
    pub fn new(mut bus: SystemEventBus) {
        spawn(async move {
            loop {
                info!("{:?}", bus.recv().await);
            }
        });
    }
}