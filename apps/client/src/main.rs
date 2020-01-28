// @todo ensure controller's protocol version matches ours

#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_ascription)]

mod app;
mod controller;
mod definition;
mod experiment;
mod report;
mod runner;

#[tokio::main]
async fn main() {
    use anyhow::*;
    use app::*;
    use structopt::StructOpt;
    use std::process::exit;

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
