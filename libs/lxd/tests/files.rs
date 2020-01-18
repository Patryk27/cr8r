use std::env::temp_dir;

use pretty_assertions::assert_eq;
use tokio::fs;

pub mod utils;

#[tokio::test]
#[ignore]
async fn files_can_be_pushed_into_containers() {
    let container_file = "/tmp/lxd-test.tmp";

    let host_file = temp_dir()
        .join("lxd-test.tmp");

    let lxd = utils::lxd()
        .await;

    utils::run(&lxd, async {
        // Step 1: Create a new file inside the container (so that we always know its contents)
        lxd.exec(
            &utils::container(),
            &["bash", "-c", &format!("echo 'Hello, World!' > {}", container_file)],
        ).await?;

        // Step 2: Pull file from the container into the host
        lxd.file_pull(
            &utils::container(),
            container_file,
            &host_file,
        ).await?;

        // Step 3: Ensure contents match
        let result = fs::read_to_string(host_file)
            .await?;

        assert_eq!(result, "Hello, World!\n");

        Ok(())
    }).await
}

#[tokio::test]
#[ignore]
async fn files_can_be_pulled_from_containers() {
    let container_file = "/tmp/lxd-test.tmp";

    let host_file = temp_dir()
        .join("lxd-test.tmp");

    let lxd = utils::lxd()
        .await;

    utils::run(&lxd, async {
        // Step 1: Create a new file inside the host (so that we always know its contents)
        fs::write(&host_file, "Hello, World!")
            .await
            .expect("Could not create a temporary file for test");

        // Step 2: Push file from the host into the container
        lxd.file_push(
            &utils::container(),
            host_file,
            container_file,
        ).await?;

        // Step 3: Ensure contents match
        let result = lxd.exec(
            &utils::container(),
            &["bash", "-c", &format!("cat {}", container_file)],
        ).await?;

        assert_eq!(result, "Hello, World!\n");

        Ok(())
    }).await
}
