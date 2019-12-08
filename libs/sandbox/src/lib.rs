#![feature(crate_visibility_modifier)]

use std::convert::TryInto;
use std::sync::Arc;

use lib_lxd::{LxdClient, LxdContainerName};

pub use self::{
    error::*,
    listener::*,
    mount::*,
    provider::*,
};

mod commands;
mod error;
mod listener;
mod mount;
mod provider;

pub struct Sandbox {
    lxd: Arc<LxdClient>,
    container: LxdContainerName,
    listener: SandboxListener,
    mount_idx: usize,
}

impl Sandbox {
    pub fn new(lxd: Arc<LxdClient>, name: String) -> Self {
        Self {
            lxd,
            container: name.try_into().unwrap(),
            listener: SandboxListener::default(),
            mount_idx: 0,
        }
    }

    pub fn set_listener(&mut self, listener: SandboxListener) {
        self.listener = listener;
    }

    // All commands are located inside the `commands` module
}
