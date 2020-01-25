use anyhow::*;
use log::*;

use lib_lxd::LxdContainerConfig;

use crate::engines::LxdSandboxEngine;

pub async fn set_env(engine: &mut LxdSandboxEngine, key: &str, value: &str) -> Result<()> {
    trace!("Executing: set_env(key=`{}`, value=`{}`)", key, value);

    engine.client.config(&engine.config.container, LxdContainerConfig::Set {
        key: format!("environment.{}", key),
        value: value.to_string(),
    }).await?;

    Ok(())
}

// @todo `key` should be ^[a-zA-Z0-9_]*$
pub async fn get_env(engine: &mut LxdSandboxEngine, key: &str) -> Result<String> {
    trace!("Executing: get_env(key=`{})`", key);

    let value = engine.client
        .exec(&engine.config.container, &["bash", "-c", &format!("echo ${}", key)])
        .await?;

    trace!(".. ok: {}", value);

    Ok(value)
}

pub fn get_host_env(key: &str) -> Result<String> {
    trace!("Executing: get_host_env(key=`{}`)", key);

    let value = std::env::var(key)
        .with_context(|| format!("Could not read host environmental variable: {}", key))?;

    trace!(".. ok: {}", value);

    Ok(value)
}