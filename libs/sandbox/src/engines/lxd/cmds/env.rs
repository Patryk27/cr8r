use anyhow::Result;

use lib_lxd::LxdContainerConfig;

use crate::LxdEngine;

pub async fn set_env(engine: &mut LxdEngine, key: &str, value: &str) -> Result<()> {
    engine.lxd.config(&engine.container, LxdContainerConfig::Set {
        key: format!("environment.{}", key),
        value: value.to_string(),
    }).await?;

    Ok(())
}

pub async fn get_env(engine: &mut LxdEngine, key: &str) -> Result<String> {
    // @todo `key` should be ^[a-zA-Z0-9_]*$

    let value = engine.lxd
        .exec(&engine.container, &["bash", "-c", &format!("echo ${}", key)])
        .await?;

    Ok(value)
}

pub fn get_host_env(key: &str) -> Result<String> {
    let value = std::env::var(key)?;

    Ok(value)
}