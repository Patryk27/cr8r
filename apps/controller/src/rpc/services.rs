use anyhow::Error;
use tonic::Status;

pub use self::{
    assignments::*,
    attachments::*,
    controller::*,
    events::*,
    experiments::*,
    jobs::*,
    reports::*,
    runners::*,
};

mod assignments;
mod attachments;
mod controller;
mod events;
mod experiments;
mod jobs;
mod reports;
mod runners;

fn transform_error(err: Error) -> Status {
    // @todo we could return more contextual status codes
    Status::unknown(err.to_string())
}
