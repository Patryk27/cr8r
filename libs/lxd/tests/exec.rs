use pretty_assertions::assert_eq;

pub mod utils;

#[tokio::test]
#[ignore]
async fn executing_bash_returns_hello_world() {
    let lxd = utils::client()
        .await;

    utils::run(&lxd, async {
        let result = lxd.exec(&utils::container(), &["bash", "-c", "echo 'Hello, World!'"])
            .await?;

        assert_eq!(result, "Hello, World!\n");

        Ok(())
    }).await
}

#[tokio::test]
#[ignore]
async fn executing_uname_returns_gnu_linux() {
    let lxd = utils::client()
        .await;

    utils::run(&lxd, async {
        let result = lxd.exec(&utils::container(), &["uname", "-a"])
            .await?;

        assert!(result.ends_with("GNU/Linux\n"));

        Ok(())
    }).await
}
