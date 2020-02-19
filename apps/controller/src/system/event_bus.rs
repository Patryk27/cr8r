use tokio::sync::broadcast::channel;

use lib_core_channel::{BrRx, BrTx};

use crate::system::SystemEvent;

pub struct SystemEventBus {
    tx: BrTx<SystemEvent>,
    rx: Option<BrRx<SystemEvent>>,
}

impl SystemEventBus {
    pub fn new() -> Self {
        let (tx, _) = channel(32);

        Self { tx, rx: None }
    }

    pub fn emit(&self, msg: SystemEvent) {
        let _ = self.tx.send(msg);
    }

    pub async fn recv(&mut self) -> SystemEvent {
        let tx = &self.tx;

        let rx = self.rx.get_or_insert_with(|| {
            tx.subscribe()
        });

        loop {
            if let Ok(msg) = rx.recv().await {
                return msg;
            }
        }
    }
}

impl Clone for SystemEventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: None,
        }
    }
}
