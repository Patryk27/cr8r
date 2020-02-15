use anyhow::*;

use lib_interop::proto::services::{PFindJobsReply, PFindJobsRequest};

use crate::system::Experiments;

pub async fn find_jobs(
    experiments: &Experiments,
    request: PFindJobsRequest,
) -> Result<PFindJobsReply> {
    unimplemented!()
}