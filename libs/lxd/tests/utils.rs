use std::future::Future;

use anyhow::Result;

use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

pub async fn run(
    lxd: &LxdClient,
    test: impl Future<Output=Result<()>>,
) {
    lxd.launch(&image(), &container())
        .await
        .expect("Could not launch the container");

    let result = test.await;

    lxd.delete(&container())
        .await
        .expect("Could not delete the container");

    result.unwrap();
}

pub async fn lxd() -> LxdClient {
    LxdClient::autodetect()
        .await
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
