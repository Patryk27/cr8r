use std::path::PathBuf;

use anyhow::{Context, Result};

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_package(
        &mut self,
        project: String,
        pkg_name: String,
        pkg_version: String,
    ) -> Result<()> {
        let cargo_path = PathBuf::from(project)
            .join("Cargo.toml");

        let cargo = self.sandbox
            .fs_read(&cargo_path)
            .await
            .context("Could not read `Cargo.toml`")?;

        let mut cargo = cargo_toml::Manifest::from_str(&cargo)
            .context("Could not parse `Cargo.toml`")?;

        let patches = cargo.patch
            .entry("crates-io".to_string())
            .or_default();

        patches.insert(pkg_name, cargo_toml::Dependency::Simple(pkg_version));

        panic!("{}", toml::to_string(&cargo)?);
    }
}
