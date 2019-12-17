//! We're totally dependent on `tonic`, which is pinned onto `tokio` `0.2.0-alpha.6`, which hasn't been implemented yet
//! any async process-related facilities, so we have to roll our own ones.
//!
//! @todo Remove all this after migrating to newer `tonic`

use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

use futures_channel::mpsc;

pub type ProcessEventTx = mpsc::UnboundedSender<ProcessEvent>;
pub type ProcessEventRx = mpsc::UnboundedReceiver<ProcessEvent>;

pub enum ProcessEvent {
    Exited {
        status: ExitStatus,
    },

    Printed {
        line: String,
    },
}

pub fn spawn(mut cmd: Command) -> ProcessEventRx {
    let mut cmd = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout
        .take()
        .map(BufReader::new);

    let stderr = cmd.stderr
        .take()
        .map(BufReader::new);

    let (tx, rx) = mpsc::unbounded();
    let tx2 = tx.clone();

    if let Some(stdout) = stdout {
        thread::spawn(move || {
            for line in stdout.lines() {
                if let Ok(line) = line {
                    let _ = tx.unbounded_send(ProcessEvent::Printed { line });
                }
            }

            let status = cmd
                .wait()
                .unwrap();

            let _ = tx.unbounded_send(ProcessEvent::Exited { status });
        });
    }

    if let Some(stderr) = stderr {
        thread::spawn(move || {
            for line in stderr.lines() {
                if let Ok(line) = line {
                    let _ = tx2.unbounded_send(ProcessEvent::Printed { line });
                }
            }
        });
    }

    rx
}
