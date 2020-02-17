#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

#[allow(dead_code)]
mod build;

mod config;
mod rpc;
mod system;

#[tokio::main]
async fn main() {
    use anyhow::*;
    use lib_core_ui::*;
    use std::process::exit;
    use self::config::*;

    let result = try {
        lib_core_log::init()
            .context("Could not initialize logger")?;

        let config = Config::load()
            .context("Could not load configuration (from `runner.yaml`)")?;

        system::start(config)
            .await?
    }: Result<()>;

    if let Err(err) = result {
        ErrorWidget::new(err)
            .eprint();

        exit(1);
    }
}