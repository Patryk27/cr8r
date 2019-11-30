#![feature(try_blocks)]
#![feature(type_ascription)]

use std::time::Duration;

use colored::Colorize;
use log::*;
use snafu::ResultExt;
use tokio::timer;

use lib_protocol::core::{ExperimentStartedReport, Report};
use lib_protocol::core::report::ReportInner;
use lib_protocol::runner::*;
use lib_protocol::runner::client::RunnerClient;

pub use self::{
    config::Config,
    error::{Error, Result, StdResult},
};

mod config;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    lib_log::init()
        .context(error::FailedToConfigure)?;

    let config = config::load()?;

    // Initialize the client
    info!("Connecting to controller at `{}`.", config.controller.address);

    let mut client = RunnerClient::connect(config.controller.address)
        .await?;

    // Ensure we're compatible with the controller
    info!("Connection acquired, checking compatibility.");

    let reply = client.hello(HelloRequest {})
        .await?
        .into_inner();

    debug!("... Controller\'s version: {}", reply.version);
    debug!("... Ok, we should be compatible."); // @todo

    // Register us as a runner
    info!("Compatibility confirmed, registering.");

    let runner_id = client
        .register(RegisterRequest {
            name: config.runner.name,
            secret: config.controller.secret,
        })
        .await?
        .into_inner()
        .id;

    // We're ready!
    info!("{}", "Registered, runner\'s ready!".green());

    let assignment = loop {
        debug!("Polling controller for an experiment.");

        let reply = client.request_experiment(RequestExperimentRequest { runner_id: runner_id.clone() })
            .await?
            .into_inner();

        if let Some(assignment) = reply.assignment {
            debug!("We\'ve been assigned an experiment, yay! {:#?}", assignment);

            // Confirm we've started the experiment
            client.report_experiment(ReportExperimentRequest {
                runner_id: runner_id.clone(),
                experiment_id: assignment.experiment_id.clone(),
                report: Some(Report {
                    report_inner: Some(ReportInner::Started(
                        ExperimentStartedReport {},
                    ))
                }),
            });

            break assignment;
        }

        timer::delay_for(Duration::from_secs(1)).await;
    };

    unimplemented!();

    Ok(())
}