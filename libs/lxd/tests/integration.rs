//! This file contains a handful of integration tests validating correctness of the LXD client.
//!
//! They all require a valid LXD instance to work and thus they are all `ignore`d by the default; you can run them from
//! the main directory using:
//!
//! ```shell,ignore
//! $ cargo test -p=lib_lxd -- --ignored --test-threads=1
//! ```

use std::future::Future;
use std::path::PathBuf;

use anyhow::Result;

use lib_lxd::{LxdClient, LxdContainerName, LxdImageName};

#[tokio::test]
#[ignore]
async fn launch_and_delete() {
    let lxd = lxd()
        .await;

    perform_test(&lxd, async {
        Ok(())
    }).await
}

#[tokio::test]
#[ignore]
async fn exec() {
    let lxd = lxd()
        .await;

    perform_test(&lxd, async {
        let result = lxd.exec(&container(), &["uname", "-a"])
            .await?;

        assert!(result.ends_with("GNU/Linux\n"));

        Ok(())
    }).await
}

async fn perform_test(
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

async fn lxd() -> LxdClient {
    LxdClient::autodetect()
        .await
        .unwrap()
}

fn container() -> LxdContainerName {
    "lib-lxd-test"
        .parse()
        .unwrap()
}

fn image() -> LxdImageName {
    "ubuntu:18.04"
        .parse()
        .unwrap()
}
