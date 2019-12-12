use std::collections::VecDeque;

use futures_channel::oneshot;
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::p_report::*;
use lib_protocol::core::PReport;

use crate::backend::experiment_watcher::ExperimentWatcherRx;

pub struct ExperimentWatcherActor {
    rx: ExperimentWatcherRx,
    pub(super) alive: bool,
    pub(super) reports: VecDeque<PReport>,
    pub(super) pending_get_tx: Option<oneshot::Sender<Option<String>>>,
}

impl ExperimentWatcherActor {
    pub fn new(rx: ExperimentWatcherRx) -> Self {
        Self {
            rx,
            alive: true,
            reports: VecDeque::with_capacity(16),
            pending_get_tx: None,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    pub(super) fn render_report(report: PReport) -> String {
        if let Some(op) = report.op {
            let op = match op {
                Op::Ping(_) => {
                    String::default()
                }

                Op::Message(PMessage { message }) => {
                    format!("(msg) {}", message)
                }

                Op::ProcessStdout(PProcessStdout { line }) => {
                    format!("(stdout) {}", line)
                }

                Op::ProcessStderr(PProcessStderr { line }) => {
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

                Op::ScenarioCompleted(PScenarioCompleted { success }) => {
                    format!("(sys) Scenario completed (result: {})", if success { "success" } else { "failure" })
                }
            };

            format!("<{}> {}", report.created_at, op)
        } else {
            String::default()
        }
    }
}