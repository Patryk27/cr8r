use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

use futures_channel::mpsc;

use crate::Result;

#[derive(Clone)]
pub struct LxdClient {
    path: PathBuf,
}

impl LxdClient {
    pub fn new() -> Self {
        Self {
            // @todo auto-detect
            path: PathBuf::from("/snap/bin/lxc"),
        }
    }

    pub fn launch(&self, container: &str, image: &str) -> Result<LxdProcessRx> {
        self.run(&[
            "launch",
            image,
            container,
            "-c",
            "security.nesting=true",
        ])
    }

    pub fn delete(&self, container: &str) -> Result<LxdProcessRx> {
        self.run(&[
            "delete",
            container,
            "--force",
        ])
    }

    pub fn list(&self) -> Result<ExitStatus> {
        unimplemented!()
    }

    pub fn exec(&self, container: &str, command: &[&str]) -> Result<LxdProcessRx> {
        let mut args = vec![
            "exec", container, "--",
        ];

        args.extend_from_slice(command);

        self.run(&args)
    }

    fn run(&self, args: &[&str]) -> Result<LxdProcessRx> {
        // Because we're totally dependent on `tonic`, we gotta stay pinned to tokio `0.2.0-alpha.6` (at least for
        // now, which doesn't provide any async cross-process-related facilities and so we have to create our own ones

        let mut cmd = Command::new("/snap/bin/lxc")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = cmd.stdout
            .take()
            .map(BufReader::new)
            .unwrap();

        let stderr = cmd.stderr
            .take()
            .map(BufReader::new)
            .unwrap();

        let (tx, rx) = mpsc::unbounded();
        let tx2 = tx.clone();

        // Spawn the `stdout` stream
        // @todo it mustn't crash
        thread::spawn(move || {
            for line in stdout.lines() {
                if let Ok(line) = line {
                    let _ = tx.unbounded_send(LxdProcessMsg::Stdout {
                        line,
                    });
                }
            }

            let status = cmd
                .wait()
                .unwrap();

            let _ = tx.unbounded_send(LxdProcessMsg::Exited {
                status,
            });
        });

        // Spawn the `stderr` stream
        thread::spawn(move || {
            for line in stderr.lines() {
                if let Ok(line) = line {
                    let _ = tx2.unbounded_send(LxdProcessMsg::Stderr {
                        line,
                    });
                }
            }
        });

        Ok(rx)
    }
}

pub type LxdProcessTx = mpsc::UnboundedSender<LxdProcessMsg>;
pub type LxdProcessRx = mpsc::UnboundedReceiver<LxdProcessMsg>;

pub enum LxdProcessMsg {
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