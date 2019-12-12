use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use futures_channel::mpsc;

use crate::{LxdClient, LxdResponseEvent, LxdResponseStream, Result};

impl LxdClient {
    crate fn invoke(&self, args: &[String]) -> Result<LxdResponseStream> {
        // Because we're totally dependent on `tonic`, we gotta stay pinned to tokio `0.2.0-alpha.6` (at least for
        // now), which doesn't provide any async cross-process-related facilities

        let mut cmd = Command::new(&self.path)
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

        thread::spawn(move || {
            for line in stdout.lines() {
                if let Ok(line) = line {
                    let _ = tx.unbounded_send(LxdResponseEvent::Stdout {
                        line,
                    });
                }
            }

            let status = cmd
                .wait()
                .unwrap();

            let _ = tx.unbounded_send(LxdResponseEvent::Exited {
                status,
            });
        });

        thread::spawn(move || {
            for line in stderr.lines() {
                if let Ok(line) = line {
                    let _ = tx2.unbounded_send(LxdResponseEvent::Stderr {
                        line,
                    });
                }
            }
        });

        Ok(LxdResponseStream::new(rx))
    }
}