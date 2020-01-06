use std::collections::HashMap;

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
        let toolchain = parse_toolchain(self.toolchain);
        let packages = parse_packages(self.packages)?;

        Ok(DDefinition {
            toolchain,
            packages,
        })
    }
}

fn parse_toolchain(toolchain: Option<String>) -> Option<DToolchain> {
    toolchain.map(|toolchain| {
        DToolchain { version: toolchain }
    })
}

fn parse_packages(packages: Vec<String>) -> Result<HashMap<String, DPackage>> {
    let mut map = HashMap::new();

    let list = packages
        .into_iter()
        .map(|package| {
            parse_package(&package)
                .with_context(|| format!("Could not parse `{}` as a package definition", package))
        })
        .collect(): Result<Vec<_>>;

    for (pkg_name, pkg_model) in list? {
        if map.contains_key(&pkg_name) {
            return Err(anyhow!("Package `{}` has been defined many times", pkg_name));
        }

        map.insert(pkg_name, pkg_model);
    }

    Ok(map)
}

fn parse_package(package: &str) -> Result<(String, DPackage)> {
    let parts = package
        .splitn(2, '=')
        .map(str::trim)
        .collect(): Vec<_>;

    if parts.len() != 2 {
        return Err(anyhow!("Invalid format - expected a key-value pair, e.g.: `anyhow = \"1.0\"`"));
    }

    let pkg_name = parts[0].to_string();

    let pkg_model = DPackage::Overridden {
        version: parts[1].to_string(),
    };

    Ok((pkg_name, pkg_model))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_toolchain {
        use super::*;

        #[test]
        fn test_none() {
            assert_eq!(
                parse_toolchain(None),
                None,
            );
        }

        #[test]
        fn test_nightly() {
            let input = "nightly".to_string();

            let expected_output = DToolchain {
                version: "nightly".to_string(),
            };

            assert_eq!(
                parse_toolchain(Some(input)).unwrap(),
                expected_output,
            );
        }
    }

    mod parse_package {
        use super::*;

        #[test]
        fn test_overridden_simple() {
            let input = "tokio = \"0.2\"";

            let expected_output = ("tokio".to_string(), DPackage::Overridden {
                version: "\"0.2\"".to_string(),
            });

            assert_eq!(
                parse_package(input).unwrap(),
                expected_output,
            );
        }

        #[test]
        fn test_overridden_advanced() {
            let input = "tokio = { version = \"0.2\", features = [\"full\"] }";

            let expected_output = ("tokio".to_string(), DPackage::Overridden {
                version: "{ version = \"0.2\", features = [\"full\"] }".to_string(),
            });

            assert_eq!(
                parse_package(input).unwrap(),
                expected_output,
            );
        }
    }
}