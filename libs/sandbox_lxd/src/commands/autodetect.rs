use std::path::Path;

use crate::{Error, LxdClient, Result};

impl LxdClient {
    pub fn autodetect() -> Result<Self> {
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

            if path.metadata().is_ok() {
                return Ok(Self::new(path));
            }
        }

        Err(Error::ClientNotFound)
    }
}

