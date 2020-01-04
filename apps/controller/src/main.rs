#![feature(box_syntax)]
#![feature(decl_macro)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(type_ascription)]

use anyhow::Result;

use self::config::*;

mod backend;
mod config;
mod frontend;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()?;

    let config = Config::load()?;
    let system = backend::start(config.ecosystem)?;

    frontend::start(
        config.controller.listen,
        config.controller.secret,
        system,
    ).await?;

    Ok(())
}