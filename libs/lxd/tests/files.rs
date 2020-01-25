use std::env::temp_dir;
use std::path::PathBuf;

use anyhow::*;
use pretty_assertions::assert_eq;
use tokio::fs;

pub mod utils;

#[tokio::test]
#[ignore]
async fn file_can_be_pulled_from_container() {
    let (container_file, host_file) = files();

    let client = utils::client()
        .await;

    utils::run(&client, async {
        let _ = fs::remove_file(&host_file)
            .await;

        // Step 1: Create a fixture-file inside the container
        client
            .exec(
                &utils::container(),
                &["bash", "-c", &format!("echo 'Hello, World!' > {}", container_file)],
            )
            .await
            .context("Step 1 failed")?;

        // Step 2: Pull fixture from container into the host
        client
            .file_pull(
                &utils::container(),
                container_file,
                &host_file,
            )
            .await
            .context("Step 2 failed")?;

        // Step 3: Ensure contents match
        let result = fs::read_to_string(host_file)
            .await
            .context("Step 3 failed")?;

        assert_eq!(result, "Hello, World!\n");

        Ok(())
    }).await
}

#[tokio::test]
#[ignore]
async fn file_can_be_pushed_into_container() {
    let (container_file, host_file) = files();

    let client = utils::client()
        .await;

    utils::run(&client, async {
        let _ = fs::remove_file(&host_file)
            .await;

        // Step 1: Create a fixture-file inside the host
        fs::write(&host_file, "Hello, World!")
            .await
            .context("Step 1 failed")?;

        // Step 2: Push fixture from host into the container
        client
            .file_push(
                &utils::container(),
                host_file,
                container_file,
            )
            .await
            .context("Step 2 failed")?;

        // Step 3: Ensure contents match
        let result = client
            .exec(
                &utils::container(),
                &["bash", "-c", &format!("cat {}", container_file)],
            )
            .await
            .context("Step 3 failed")?;

        assert_eq!(result, "Hello, World!\n");

        Ok(())
    }).await
}

fn files() -> (&'static str, PathBuf) {
    (
        "/root/lxd-test.tmp",
        temp_dir().join("lxd-test.tmp"),
    )
}