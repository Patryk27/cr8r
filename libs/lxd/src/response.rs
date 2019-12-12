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

    Stderr {
        line: String,
    },

    Stdout {
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

    pub async fn stdout(mut self) -> Result<String> {
        let mut stdout = String::new();

        while let Some(event) = self.next().await {
            match event {
                LxdResponseEvent::Exited { status } => {
                    return if status.success() {
                        Ok(stdout)
                    } else {
                        Err(Error::CommandTerminatedAbruptly)
                    };
                }

                LxdResponseEvent::Stdout { line } => {
                    stdout.push_str(&line);
                    stdout.push('\n');
                }

                _ => (),
            }
        }

        Err(Error::CommandTerminatedAbruptly)
    }

    pub fn stdout_sync(self) -> Result<String> {
        let (tx, rx) = std_mpsc::sync_channel(1);

        tokio::spawn(async move {
            let stdout = self.stdout().await;

            tx.send(stdout).unwrap();
        });

        rx.recv().unwrap_or_else(|_| Err(Error::CommandTerminatedAbruptly))
    }

    pub async fn wait(self) -> Result<()> {
        self.stdout()
            .await
            .map(|_| ())
    }

    pub fn wait_sync(self) -> Result<()> {
        self.stdout_sync()
            .map(|_| ())
    }
}
