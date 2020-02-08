use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type UTx<T> = UnboundedSender<T>;
pub type URx<T> = UnboundedReceiver<T>;

pub trait Notify {
    fn notify(self, tx: &UTx<Self>) where Self: Sized;
}

impl<T> Notify for T {
    fn notify(self, tx: &UTx<Self>) {
        let _ = tx.send(self);
    }
}