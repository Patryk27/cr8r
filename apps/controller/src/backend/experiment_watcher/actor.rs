use std::collections::VecDeque;

use futures_channel::oneshot;
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{Report, report};

use crate::backend::{ExperimentWatcherMsg, ExperimentWatcherRx};

pub struct ExperimentWatcherActor {
    rx: ExperimentWatcherRx,
    alive: bool,
    reports: VecDeque<Report>,
    pending_tx: Option<oneshot::Sender<Option<String>>>,
}

impl ExperimentWatcherActor {
    pub fn new(rx: ExperimentWatcherRx) -> Self {
        Self {
            rx,
            alive: true,
            reports: VecDeque::with_capacity(16),
            pending_tx: None,
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started, entering event loop");

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            match msg {
                ExperimentWatcherMsg::Add { report } => {
                    self.add(report);
                }

                ExperimentWatcherMsg::Kill => {
                    self.kill();
                }

                ExperimentWatcherMsg::Get { tx } => {
                    self.get(tx);
                }
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn add(&mut self, report: Report) {
        if !self.alive {
            return;
        }

        if let Some(tx) = self.pending_tx.take() {
            let _ = tx.send(Some(
                Self::print_report(report)
            ));
        } else {
            self.reports.push_back(report);
        }
    }

    fn kill(&mut self) {
        self.alive = false;

        if let Some(tx) = self.pending_tx.take() {
            let _ = tx.send(None);
        }
    }

    fn get(&mut self, tx: oneshot::Sender<Option<String>>) {
        if let Some(report) = self.reports.pop_front() {
            let _ = tx.send(Some(
                Self::print_report(report)
            ));
        } else {
            self.pending_tx = Some(tx);
        }
    }

    fn print_report(report: Report) -> String {
        use report::*;

        if let Some(op) = report.op {
            let op = match op {
                Op::Ping(_) => {
                    String::default()
                }

                Op::Message(Message { message }) => {
                    format!("(msg) {}", message)
                }

                Op::ProcessStdout(ProcessStdout { line }) => {
                    format!("(stdout) {}", line)
                }

                Op::ProcessStderr(ProcessStderr { line }) => {
                    format!("(stderr) {}", line)
                }

                Op::ExperimentStarted(_) => {
                    "(sys) Experiment started".to_string()
                }

                Op::ExperimentCompleted(_) => {
                    "(sys) Experiment completed".to_string()
                }

                Op::ScenarioStarted(_) => {
                    "(sys) Scenario started".to_string()
                }

                Op::ScenarioCompleted(ScenarioCompleted { success }) => {
                    format!("(sys) Scenario completed (result: {})", if success { "success" } else { "failure" })
                }
            };

            format!("<{}> {}", report.created_at, op)
        } else {
            String::default()
        }
    }
}