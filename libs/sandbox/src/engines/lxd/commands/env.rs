use anyhow::{Context, Result};
use log::*;

use lib_lxd::LxdContainerConfig;

use crate::engines::LxdEngine;

pub async fn set_env(engine: &mut LxdEngine, key: &str, value: &str) -> Result<()> {
    debug!("set_env :: key={}, value={}", key, value);

    engine.client.config(&engine.container, LxdContainerConfig::Set {
        key: format!("environment.{}", key),
        value: value.to_string(),
    }).await?;

    Ok(())
}

// @todo `key` should be ^[a-zA-Z0-9_]*$
pub async fn get_env(engine: &mut LxdEngine, key: &str) -> Result<String> {
    debug!("get_env :: key={}", key);

    let value = engine.client
        .exec(&engine.container, &["bash", "-c", &format!("echo ${}", key)])
        .await?;

    debug!("... = {}", value);

    Ok(value)
}

pub fn get_host_env(key: &str) -> Result<String> {
    debug!("get_host_env :: key={}", key);

    let value = std::env::var(key)
        .with_context(|| format!("Could not read host environmental variable: {}", key))?;

    debug!("... = {}", value);

    Ok(value)
}