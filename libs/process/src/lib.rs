use std::io;
use std::process::{ExitStatus, Stdio};

use tokio::io::AsyncRead;
use tokio::process::{Child, Command};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_util::codec::{FramedRead, LinesCodec};

pub enum ProcessEvent {
    Crashed {
        err: io::Error,
    },

    Exited {
        status: ExitStatus,
    },

    Printed {
        line: String,
    },
}

pub fn spawn(mut cmd: Command) -> impl Stream<Item=ProcessEvent> {
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap(); // @todo

    let (tx, rx) = mpsc::unbounded_channel();

    if let Some(stdout) = child.stdout().take() {
        spawn_streamer(tx.clone(), stdout);
    }

    if let Some(stderr) = child.stderr().take() {
        spawn_streamer(tx.clone(), stderr);
    }

    spawn_watcher(tx, child);

    rx
}

fn spawn_streamer(tx: mpsc::UnboundedSender<ProcessEvent>, stream: impl AsyncRead + Unpin + Send + 'static) {
    let mut stream = FramedRead::new(stream, LinesCodec::new());

    tokio::spawn(async move {
        while let Some(line) = stream.next().await {
            if let Ok(line) = line {
                let _ = tx.send(ProcessEvent::Printed { line });
            }
        }
    });
}

fn spawn_watcher(tx: mpsc::UnboundedSender<ProcessEvent>, child: Child) {
    tokio::spawn(async move {
        match child.await {
            Ok(status) => {
                let _ = tx.send(ProcessEvent::Exited { status });
            }

            Err(err) => {
                let _ = tx.send(ProcessEvent::Crashed { err });
            }
        }
    });
}