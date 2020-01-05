use std::convert::TryInto;

use structopt::StructOpt;

use anyhow::{anyhow, Context, Result};
use lib_interop::domain::DDefinition;
use lib_interop::domain::definition_inner::*;

#[derive(Debug, StructOpt)]
pub struct Definition {
    #[structopt(long = "toolchain")]
    toolchain: Option<String>,

    #[structopt(long = "pkg")]
    packages: Vec<String>,
}

impl Definition {
    pub fn parse(self) -> Result<DDefinition> {
        let mut def = DDefinition::default();

        parse_toolchain(&mut def, self.toolchain);
        parse_packages(&mut def, self.packages)?;

        Ok(def)
    }
}

fn parse_toolchain(def: &mut DDefinition, toolchain: Option<String>) {
    if let Some(toolchain) = toolchain {
        def.overridden_toolchain = Some(DOverriddenToolchain { toolchain });
    }
}

fn parse_packages(def: &mut DDefinition, packages: Vec<String>) -> Result<()> {
    for package in packages {
        parse_package(def, &package)
            .with_context(|| format!("Could not parse `{}` as a package definition", package))?;
    }

    Ok(())
}

fn parse_package(def: &mut DDefinition, package: &str) -> Result<()> {
    let mut parts = package
        .splitn(1, '=')
        .collect(): Vec<_>;

    if parts.len() != 2 {
        return Err(anyhow!("Invalid format - expected a key-value pair, e.g.: `anyhow = \"1.0\"`"));
    }

    def.overridden_packages.push(DOverriddenPackage {
        name: parts[0].to_string(),
        version: parts[1].to_string(),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toolchain() {
        let mut def = DDefinition::default();

        // Case: toolchain not set
        parse_toolchain(&mut def, None);

        assert_eq!(def.overridden_toolchain, None);

        // Case: toolchain set
        parse_toolchain(&mut def, Some("nightly".to_string()));

        assert_eq!(def.overridden_toolchain, Some(DOverriddenToolchain {
            toolchain: "nightly".to_string(),
        }));
    }

    #[test]
    fn test_parse_package() {
        // @todo
    }
}