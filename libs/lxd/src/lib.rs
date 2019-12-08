#![feature(crate_visibility_modifier)]

use std::path::{Path, PathBuf};

pub use self::{
    error::*,
    event::*,
    models::*,
};

mod commands;
mod error;
mod event;
mod models;

pub struct LxdClient {
    path: PathBuf,
}

impl LxdClient {
    pub fn new(path: &Path) -> Self {
        Self { path: path.into() }
    }

    // All commands are located inside the `commands` module
}