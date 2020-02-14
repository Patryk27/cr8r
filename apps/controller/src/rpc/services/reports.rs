use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::reports_server::Reports;

use crate::system;

pub struct ReportsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Reports for ReportsService {
    async fn find_reports(
        &self,
        request: Request<PFindReportsRequest>,
    ) -> Result<Response<PFindReportsReply>, Status> {
        unimplemented!()
    }
}
