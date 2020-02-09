#![feature(box_syntax)]
#![feature(decl_macro)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(type_ascription)]

mod system;
mod config;
mod rpc;

#[tokio::main]
async fn main() {
    use anyhow::*;
    use std::process::exit;
    use self::config::*;

    let result = try {
        lib_core_log::init()
            .context("Could not initialize logging facility")?;

        let config = Config::load()
            .context("Could not load configuration from `controller.yaml`")?;

        let system = system::start(config.ecosystem)
            .context("Could not start controller (sys module failed)")?;

        rpc::start(config.controller.address, config.controller.secret, system)
            .await
            .context("Could not start controller (rpc module failed)")?
    }: Result<()>;

    if let Err(err) = result {
        eprintln!("{}", lib_core_ui::ErrorWidget::new(&err));

        exit(1);
    }
}