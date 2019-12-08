use std::process::ExitStatus;

use futures_channel::mpsc;

pub type LxdEventTx = mpsc::UnboundedSender<LxdEvent>;
pub type LxdEventRx = mpsc::UnboundedReceiver<LxdEvent>;

pub enum LxdEvent {
    Exited {
        status: ExitStatus,
    },

    Stderr {
        line: String,
    },

    Stdout {
        line: String,
    },
}