use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::reports_server::Reports;

use crate::system::ExperimentStore;

use super::transform_error;

mod find;

pub struct ReportsService {
    pub experiment_store: ExperimentStore,
}

#[tonic::async_trait]
impl Reports for ReportsService {
    async fn find_reports(
        &self,
        request: Request<PFindReportsRequest>,
    ) -> Result<Response<PFindReportsReply>, Status> {
        find::find_reports(&self.experiment_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }
}
