// @todo ensure controller's protocol version matches ours

#![feature(async_closure)]
#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

mod commands;
mod modules;
mod widgets;

#[tokio::main]
async fn main() {
    use anyhow::*;
    use lib_core_ui::*;
    use std::process::exit;
    use structopt::StructOpt;
    use self::commands::AppCommand;
    use self::modules::app::{AppConfig, AppContext};

    let result = try {
        let cmd = AppCommand::from_args();

        let config = AppConfig::load()
            .context("Could not load configuration (from `client.yaml`)")?;

        let mut ctxt = AppContext::new(config);

        cmd.run(&mut ctxt)
            .await?
    }: Result<()>;

    if let Err(err) = result {
        ErrorWidget::new(err)
            .eprint();

        exit(1);
    }
}
