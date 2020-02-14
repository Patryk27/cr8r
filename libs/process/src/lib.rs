#![feature(box_syntax)]

use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::process::{ExitStatus, Stdio};

use tokio::io::AsyncRead;
use tokio::process::Command;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::task::spawn;
use tokio_util::codec::{FramedRead, LinesCodec};

pub struct Process<'a> {
    cmd: Command,
    listener: Option<Box<dyn FnMut(String) + Send + 'a>>,
}

impl<'a> Process<'a> {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            cmd: Command::new(program),
            listener: None,
        }
    }

    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.cmd.current_dir(dir);
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item=impl AsRef<OsStr>>) -> Self {
        self.cmd.args(args);
        self
    }

    pub fn listener(mut self, listener: impl FnMut(String) + Send + 'a) -> Self {
        self.listener = Some(box listener);
        self
    }

    pub async fn spawn(mut self) -> Result<ExitStatus, io::Error> {
        let mut child = self.cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let (tx, mut rx) = unbounded_channel();

        if let Some(mut listener) = self.listener {
            if let Some(stdout) = child.stdout.take() {
                spawn_stream(tx.clone(), stdout);
            }

            if let Some(stderr) = child.stderr.take() {
                spawn_stream(tx, stderr);
            }

            while let Some(line) = rx.next().await {
                listener(line);
            }
        }

        child.await
    }
}

fn spawn_stream(tx: UnboundedSender<String>, stream: impl AsyncRead + Unpin + Send + 'static) {
    let mut stream = FramedRead::new(stream, LinesCodec::new());

    spawn(async move {
        while let Some(line) = stream.next().await {
            if let Ok(line) = line {
                if tx.send(line).is_err() {
                    return;
                }
            }
        }
    });
}