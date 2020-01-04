use std::path::Path;

use tokio::fs;

use anyhow::{anyhow, Result};

use crate::LxdClient;

pub async fn autodetect() -> Result<LxdClient> {
    let paths = [
        // LXD installed from Snap:
        "/snap/bin/lxc",

        // LXD installed from `apt` (Ubuntu):
        "/usr/bin/lxc",

        // Other possible paths, but not encountered by me in the wild:
        "/usr/local/bin/lxc",
        "/usr/local/sbin/lxc",
        "/bin/lxc",
        "/sbin/lxc",
    ];

    for path in &paths {
        let path = Path::new(path);

        if fs::metadata(path).await.is_ok() {
            return Ok(LxdClient::new(path));
        }
    }

    Err(anyhow!("Could not detect location of the `lxc` executable - please ensure you have LXD installed"))
}

