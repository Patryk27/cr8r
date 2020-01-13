use std::path::PathBuf;

use anyhow::{Context, Result};

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_dependency(
        &mut self,
        project: String,
        dep_registry: String,
        dep_name: String,
        dep_version: String,
    ) -> Result<()> {
        let cargo_path = PathBuf::from(project)
            .join("Cargo.toml");

        let cargo = self.sandbox
            .fs_read(&cargo_path)
            .await
            .context("Could not read `Cargo.toml`")?;

        let mut manifest = cargo_toml::Manifest::from_str(&cargo)
            .context("Could not parse `Cargo.toml`")?;

        let patches = manifest.patch
            .entry(dep_registry)
            .or_default();

        patches.insert(dep_name, cargo_toml::Dependency::Simple(dep_version));

        panic!("{}", toml::to_string(&manifest)?);
    }
}
