use std::collections::VecDeque;

use futures_channel::oneshot;
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::Report;

use crate::backend::{ExperimentWatcherCommand, ExperimentWatcherCommandRx};

pub struct ExperimentWatcherActor {
    alive: bool,
    reports: VecDeque<Report>,
    pending_get_tx: Option<oneshot::Sender<Option<Report>>>,
}

impl ExperimentWatcherActor {
    pub fn new() -> Self {
        Self {
            alive: true,
            reports: VecDeque::with_capacity(16),
            pending_get_tx: None,
        }
    }

    pub async fn start(mut self, mut rx: ExperimentWatcherCommandRx) {
        debug!("Actor started, entering event loop");

        while let Some(cmd) = rx.next().await {
            debug!("Processing command: {:?}", cmd);

            match cmd {
                ExperimentWatcherCommand::Add { report } => {
                    if self.alive {
                        if let Some(tx) = self.pending_get_tx.take() {
                            let _ = tx.send(Some(report));
                        } else {
                            self.reports.push_back(report);
                        }
                    }
                }

                ExperimentWatcherCommand::Kill => {
                    self.alive = false;

                    if let Some(tx) = self.pending_get_tx.take() {
                        let _ = tx.send(None);
                    }
                }

                ExperimentWatcherCommand::Get { tx } => {
                    if let Some(report) = self.reports.pop_front() {
                        let _ = tx.send(Some(report));
                    } else {
                        self.pending_get_tx = Some(tx);
                    }
                }
            }
        }

        debug!("Actor orphaned, halting it");
    }
}