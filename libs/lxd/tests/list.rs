pub mod utils;

#[tokio::test]
#[ignore]
async fn listing_all_containers_returns_our_container() {
    let lxd = utils::lxd()
        .await;

    utils::run(&lxd, async {
        let containers = lxd.list()
            .await?;

        assert!(
            containers
                .into_iter()
                .any(|container| container.name == utils::container())
        );

        Ok(())
    }).await
}