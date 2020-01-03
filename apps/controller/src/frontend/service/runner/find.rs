use lib_interop::proto::controller::PFindRunnersReply;

use crate::backend::System;

pub async fn find_runners(system: &System) -> PFindRunnersReply {
    let mut runners = Vec::new();

    for runner in system.find_runners().await {
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