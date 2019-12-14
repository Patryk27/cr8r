use std::process::ExitStatus;
use std::sync::mpsc as std_mpsc;

use futures_channel::mpsc;
use futures_util::StreamExt;

use crate::{Error, Result};

pub type LxdResponseEventTx = mpsc::UnboundedSender<LxdResponseEvent>;
pub type LxdResponseEventRx = mpsc::UnboundedReceiver<LxdResponseEvent>;

pub enum LxdResponseEvent {
    Exited {
        status: ExitStatus,
    },

    Printed {
        line: String,
    },
}

pub struct LxdResponseStream {
    rx: LxdResponseEventRx,
}

impl LxdResponseStream {
    crate fn new(rx: LxdResponseEventRx) -> Self {
        Self { rx }
    }

    pub async fn next(&mut self) -> Option<LxdResponseEvent> {
        self.rx
            .next()
            .await
    }

    pub async fn output(mut self) -> Result<String> {
        let mut output = String::new();

        while let Some(event) = self.next().await {
            match event {
                LxdResponseEvent::Exited { status } => {
                    return if status.success() {
                        Ok(output)
                    } else {
                        Err(Error::CommandTerminatedAbruptly)
                    };
                }

                LxdResponseEvent::Printed { line } => {
                    output.push_str(&line);
                    output.push('\n');
                }
            }
        }

        Err(Error::CommandTerminatedAbruptly)
    }

    pub fn output_sync(self) -> Result<String> {
        let (tx, rx) = std_mpsc::sync_channel(1);

        tokio::spawn(async move {
            let stdout = self.output().await;

            tx.send(stdout).unwrap();
        });

        rx.recv().unwrap_or_else(|_| Err(Error::CommandTerminatedAbruptly))
    }

    pub async fn wait(self) -> Result<()> {
        self.output()
            .await
            .map(|_| ())
    }

    pub fn wait_sync(self) -> Result<()> {
        self.output_sync()
            .map(|_| ())
    }
}
