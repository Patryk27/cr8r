#![feature(box_syntax)]
#![feature(decl_macro)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(type_ascription)]

use std::path::PathBuf;

use snafu::ResultExt;

mod backend;
mod core;
mod frontend;

#[tokio::main]
async fn main() -> core::Result<()> {
    lib_log::init()
        .context(core::error::CouldntStart)?;

    let config = core::config::load(
        &PathBuf::from("controller.yaml")
    )?;

    let system = backend::start(
        config.ecosystem,
    ).unwrap(); // @todo

    frontend::start(
        config.controller.listen, system,
    ).await?;

    Ok(())
}