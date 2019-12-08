use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use futures_channel::mpsc;

use crate::{LxdClient, LxdEvent, LxdEventRx, Result};

impl LxdClient {
    crate fn invoke(&self, args: &[String]) -> Result<LxdEventRx> {
        // Because we're totally dependent on `tonic`, we gotta stay pinned to tokio `0.2.0-alpha.6` (at least for
        // now), which doesn't provide any async cross-process-related facilities

        let mut cmd = Command::new("/snap/bin/lxc")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
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
        thread::spawn(move || {
            for line in stdout.lines() {
                if let Ok(line) = line {
                    let _ = tx.unbounded_send(LxdEvent::Stdout {
                        line,
                    });
                }
            }

            let status = cmd
                .wait()
                .unwrap();

            let _ = tx.unbounded_send(LxdEvent::Exited {
                status,
            });
        });

        // Spawn the `stderr` stream
        thread::spawn(move || {
            for line in stderr.lines() {
                if let Ok(line) = line {
                    let _ = tx2.unbounded_send(LxdEvent::Stderr {
                        line,
                    });
                }
            }
        });

        Ok(rx)
    }
}