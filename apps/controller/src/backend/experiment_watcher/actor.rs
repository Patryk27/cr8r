use std::collections::VecDeque;

use futures_channel::oneshot;
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::p_report::*;
use lib_protocol::core::PReport;
use lib_protocol::for_client::p_watch_experiment_reply::Kind;
use lib_protocol::for_client::PWatchExperimentReply;

use crate::backend::experiment_watcher::ExperimentWatcherRx;

pub struct ExperimentWatcherActor {
    rx: ExperimentWatcherRx,
    pub(super) alive: bool,
    pub(super) pending_replies: VecDeque<PWatchExperimentReply>,
    pub(super) pending_tx: Option<oneshot::Sender<PWatchExperimentReply>>,
}

impl ExperimentWatcherActor {
    pub fn new(rx: ExperimentWatcherRx) -> Self {
        Self {
            rx,
            alive: true,
            pending_replies: VecDeque::new(),
            pending_tx: None,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    pub(super) fn add_pending_reply(&mut self, reply: PWatchExperimentReply) {
        if let Some(tx) = self.pending_tx.take() {
            let _ = tx.send(reply);
        } else {
            self.pending_replies.push_front(reply);
        }
    }

    /// Transforms `PReport` into more user-friendly-oriented `PWatchExperimentReply`.
    /// Returns `None` if this report yields no message for the user.
    pub(super) fn report_to_reply(report: &PReport) -> Option<PWatchExperimentReply> {
        let created_at = &report.created_at;

        let (kind, message) = match report.op.as_ref()? {
            Op::Ping(_) => {
                // We don't want to report pings to the user, because they convey no meaning for them - pings are only
                // for the controller

                return None;
            }

            Op::Message(PMessage { message }) => {
                (Kind::UserMessage, message.to_owned())
            }

            Op::ProcessOutput(PProcessOutput { line }) => {
                (Kind::ProcessOutput, line.to_owned())
            }

            Op::ExperimentStarted(_) => {
                (Kind::SystemMessage, "Experiment started".to_string())
            }

            Op::ExperimentCompleted(_) => {
                (Kind::SystemMessage, "Experiment completed".to_string())
            }

            Op::ExperimentAborted(_) => {
                (Kind::SystemMessage, "Experiment aborted".to_string())
            }

            Op::ScenarioStarted(_) => {
                (Kind::SystemMessage, "Scenario started".to_string())
            }

            Op::ScenarioCompleted(PScenarioCompleted { success }) => {
                let success = if *success { "success" } else { "failure" };
                (Kind::SystemMessage, format!("Scenario completed (result: {})", success))
            }
        };

        Some(PWatchExperimentReply {
            created_at: created_at.to_owned(),
            kind: kind as i32,
            message,
        })
    }
}