use lib_interop::proto::services::PFindRunnersReply;

use crate::system::System;

pub async fn find_runners(system: &System) -> PFindRunnersReply {
    let mut runners = Vec::new();

    for runner in system.runners.find_all().await {
        let runner = runner
            .get_model()
            .await;

        runners.push(runner);
    }

    runners.sort_unstable_by(|a, b| {
        a.name.cmp(&b.name)
    });

    let runners = runners
        .into_iter()
        .map(Into::into)
        .collect();

    PFindRunnersReply { runners }
}