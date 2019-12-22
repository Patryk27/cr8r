use std::path::PathBuf;

use lib_lxd::{LxdContainerName, LxdImageName};

#[derive(Clone, Debug)]
pub enum SandboxDef {
    Lxd {
        container: LxdContainerName,
        image: LxdImageName,
    },

    Shell {
        root: PathBuf,
    },
}
