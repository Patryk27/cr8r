use std::future::Future;

use anyhow::*;

use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

pub async fn run(
    lxd: &LxdClient,
    test: impl Future<Output=Result<()>>,
) {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("mio", log::LevelFilter::Error)
        .filter_module("tokio", log::LevelFilter::Error)
        .is_test(true)
        .try_init();

    lxd.launch(&image(), &container()).await
        .expect("Could not launch the container");

    let result = test.await;

    lxd.delete(&container()).await
        .expect("Could not delete the container");

    result.unwrap();
}

pub async fn client() -> LxdClient {
    LxdClient::autodetect().await
        .unwrap()
}

pub fn container() -> LxdContainerName {
    "lib-lxd-test"
        .parse()
        .unwrap()
}

pub fn image() -> LxdImageName {
    "ubuntu:18.04"
        .parse()
        .unwrap()
}
