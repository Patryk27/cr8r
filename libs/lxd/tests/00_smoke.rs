//! This file has been named as `00_smoke`, so that it's always run as the first test - if anything here fails, one can
//! quickly notice that something went totally sideways with our LXD connector.

pub mod utils;

#[tokio::test]
#[ignore]
async fn containers_can_be_launched_and_destroyed() {
    let lxd = utils::client().await;

    utils::run(&lxd, async {
        Ok(())
    }).await
}
