#![feature(box_syntax)]
#![feature(decl_macro)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(type_ascription)]

use std::process::exit;

use anyhow::{Context, Result};

use lib_error::PrintableError;

use self::config::*;

mod backend;
mod config;
mod frontend;

#[tokio::main]
async fn main() {
    let result = try {
        lib_log::init()
            .context("Could not initialize logging facility")?;

        let config = Config::load()
            .context("Could not load configuration from `controller.yaml`")?;

        let system = backend::start(config.ecosystem)
            .context("Could not start controller's backend")?;

        frontend::start(config.controller.listen, config.controller.secret, system)
            .await
            .context("Could not start controller's frontend")?
    }: Result<()>;

    if let Err(err) = result {
        err.print();
        exit(1);
    }
}