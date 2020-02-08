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
    use commands::AppCommand;
    use modules::app::{AppConfig, AppContext};
    use widgets::AppErrorWidget;
    use std::process::exit;
    use structopt::StructOpt;

    let result = try {
        let cmd = AppCommand::from_args();

        let config = AppConfig::load()
            .context("Could not load configuration from `client.yaml`")?;

        let mut ctxt = AppContext::new(config);

        cmd.run(&mut ctxt)
            .await?
    }: Result<()>;

    if let Err(err) = result {
        AppErrorWidget::print(err);

        exit(1);
    }
}
