use tokio::sync::{broadcast, mpsc, oneshot};

pub type BTx<T> = mpsc::Sender<T>;
pub type BRx<T> = mpsc::Receiver<T>;

pub type BrTx<T> = broadcast::Sender<T>;
pub type BrRx<T> = broadcast::Receiver<T>;

pub type OTx<T> = oneshot::Sender<T>;
pub type ORx<T> = oneshot::Receiver<T>;

pub type UTx<T> = mpsc::UnboundedSender<T>;
pub type URx<T> = mpsc::UnboundedReceiver<T>;

pub trait SendTo<Tx> {
    fn send_to(self, tx: Tx);
}

impl<T> SendTo<&UTx<T>> for T {
    fn send_to(self, tx: &UTx<T>) {
        let _ = tx.send(self);
    }
}

impl<T> SendTo<OTx<T>> for T {
    fn send_to(self, tx: OTx<T>) {
        let _ = tx.send(self);
    }
}
