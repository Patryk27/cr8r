use lib_interop::protocol::for_client::PFindRunnersReply;

use crate::backend::System;

pub async fn find_runners(system: &System) -> PFindRunnersReply {
    let mut runners = Vec::new();

    for runner in system.find_runners().await {
        let runner = runner
            .get_model()
            .await;

        runners.push(runner.into());
    }

    PFindRunnersReply { runners }
}