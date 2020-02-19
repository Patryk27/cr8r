use anyhow::*;

use lib_interop::proto::services::PFindRunnersReply;

use crate::system::RunnerStore;

pub async fn find_runners(runner_store: &RunnerStore) -> Result<PFindRunnersReply> {
    let mut runners = Vec::new();

    for runner in runner_store.find_all().await {
        let runner = runner.get_model().await;

        runners.push(runner);
    }

    runners.sort_unstable_by(|a, b| {
        a.name.cmp(&b.name)
    });

    let runners = runners
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(PFindRunnersReply { runners })
}