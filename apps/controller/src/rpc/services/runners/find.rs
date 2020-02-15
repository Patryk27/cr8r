use anyhow::*;

use lib_interop::proto::services::PFindRunnersReply;

use crate::system::Runners;

pub async fn find_runners(runners: &Runners) -> Result<PFindRunnersReply> {
    let mut found_runners = Vec::new();

    for runner in runners.find_all().await {
        let runner = runner
            .get_model()
            .await;

        found_runners.push(runner);
    }

    found_runners.sort_unstable_by(|a, b| {
        a.name.cmp(&b.name)
    });

    let runners = found_runners
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(PFindRunnersReply { runners })
}