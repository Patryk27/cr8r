use structopt::StructOpt;

use anyhow::{anyhow, Context, Result};
use lib_interop::domain::DDefinition;
use lib_interop::domain::definition_inner::*;

#[derive(Debug, StructOpt)]
pub struct Definition {
    #[structopt(long = "toolchain", short = "t")]
    toolchain: Option<String>,

    #[structopt(long = "dependency", short = "d")]
    dependencies: Vec<String>,
}

impl Definition {
    pub fn parse(self) -> Result<DDefinition> {
        let toolchain = parse_toolchain(self.toolchain);
        let dependencies = parse_dependencies(self.dependencies)?;

        Ok(DDefinition {
            toolchain,
            dependencies,
        })
    }
}

fn parse_toolchain(toolchain: Option<String>) -> Option<DToolchain> {
    toolchain.map(|version| {
        DToolchain { version }
    })
}

fn parse_dependencies(dependencies: Vec<String>) -> Result<Vec<DDependency>> {
    // @todo find duplicates

    dependencies
        .into_iter()
        .map(|dependency| {
            parse_dependency(&dependency)
                .with_context(|| format!("Could not understand `{}` as a dependency definition", dependency))
        })
        .collect()
}

// @todo allow patching dependencies from outside `crates.io`
fn parse_dependency(dependency: &str) -> Result<DDependency> {
    let parts = dependency
        .splitn(2, '=')
        .map(str::trim)
        .collect(): Vec<_>;

    if parts.len() != 2 {
        return Err(anyhow!("Invalid format - expected a key-value pair, e.g.: `anyhow = \"1.0\"`"));
    }

    let registry = "crates.io".to_string(); // @todo

    let name = parts[0].to_string();

    let action = DDependencyAction::Override {
        version: parts[1].to_string(),
    };

    Ok(DDependency { registry, name, action })
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

            let expected = DToolchain {
                version: "nightly".to_string(),
            };

            let actual = parse_toolchain(Some(input))
                .unwrap();

            assert_eq!(expected, actual);
        }
    }

    mod parse_dependency {
        use super::*;

        mod crates_io {
            use super::*;

            #[test]
            fn test_basic() {
                let input = "tokio = \"0.2\"";

                let expected = DDependency {
                    registry: "crates.io".to_string(),
                    name: "tokio".to_string(),

                    action: DDependencyAction::Override {
                        version: "\"0.2\"".to_string(),
                    },
                };

                let actual = parse_dependency(input)
                    .unwrap();

                assert_eq!(expected, actual);
            }

            #[test]
            fn test_advanced() {
                let input = "tokio = { version = \"0.2\", features = [\"full\"] }";

                let expected = DDependency {
                    registry: "crates.io".to_string(),
                    name: "tokio".to_string(),

                    action: DDependencyAction::Override {
                        version: "{ version = \"0.2\", features = [\"full\"] }".to_string(),
                    },
                };

                let actual = parse_dependency(input)
                    .unwrap();

                assert_eq!(expected, actual);
            }
        }

        mod git {
            use super::*;

// @todo
        }
    }
}