use std::path::{Path, PathBuf};

use anyhow::*;
use log::*;
use tokio::fs;

use lib_process::Process;

use crate::LxdListener;

pub struct LxdConnector {
    path: PathBuf,
    listener: LxdListener,
}

impl LxdConnector {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            listener: LxdListener::default(),
        }
    }

    pub fn set_listener(&mut self, listener: LxdListener) {
        self.listener = listener;
    }

    pub async fn autodetect() -> Result<Self> {
        let paths = [
            // LXD installed from Snap
            "/snap/bin/lxc",

            // LXD installed from `apt` (Ubuntu)
            "/usr/bin/lxc",

            // Other possible paths, but not encountered by me in the wild
            "/usr/local/bin/lxc",
            "/usr/local/sbin/lxc",
            "/bin/lxc",
            "/sbin/lxc",
        ];

        for path in &paths {
            let path = Path::new(path);

            if fs::metadata(path).await.is_ok() {
                return Ok(Self::new(path.into()));
            }
        }

        Err(anyhow!("Could not detect location of the `lxc` executable - please ensure you have LXD installed"))
    }

    pub async fn invoke(&self, args: &[String]) -> Result<String> {
        self.invoke_ex(args, true).await
    }

    pub async fn invoke_silent(&self, args: &[String]) -> Result<String> {
        self.invoke_ex(args, false).await
    }

    async fn invoke_ex(&self, args: &[String], listen: bool) -> Result<String> {
        trace!("Executing: invoke_ex(args=`{:?}`, listen={})", args, listen);

        let mut output = String::new();

        let status = Process::new(&self.path)
            .args(args)
            .listener(|line| {
                trace!(".. LXD says: {}", line);

                output.push_str(&line);
                output.push('\n');

                if listen {
                    if let Some(handler) = &self.listener.on_output {
                        handler(line);
                    }
                }
            })
            .spawn().await?;

        if status.success() {
            Ok(output)
        } else {
            Err(anyhow!("Previous command returned a non-zero exit code"))
        }
    }
}