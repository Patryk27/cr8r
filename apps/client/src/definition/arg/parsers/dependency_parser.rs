use anyhow::*;

use lib_interop::domain::definition_inner::{DDependencyDef, DDependencyDefSource};

pub fn parse_dependencies(deps: Vec<String>) -> Result<Vec<DDependencyDef>> {
    // @todo find duplicates

    deps.into_iter()
        .map(|dep| {
            parse_dependency(&dep)
                .with_context(|| format!("Could not understand dependency `{}`", dep))
        })
        .collect()
}

pub fn parse_dependency(dep: &str) -> Result<DDependencyDef> {
    let parts = dep
        .splitn(2, '=')
        .map(str::trim)
        .collect(): Vec<_>;

    if parts.len() != 2 {
        return Err(anyhow!("Invalid format - expected a key-value pair, e.g.: `anyhow = \"1.0\"`"));
    }

    let name = parts[0].to_string();

    let source = DDependencyDefSource::Version {
        version: parts[1].to_string(),
    };

    Ok(DDependencyDef { name, source })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_given_branch_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_branch_source() {
            assert(DDependencyDef {
                name: "tokio".to_string(),

                source: DDependencyDefSource::Branch {
                    branch: "features/performance-fix".to_string(),
                },
            }, "tokio:branch=features/performance-fix");
        }
    }

    mod when_given_tag_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_tag_source() {
            assert(DDependencyDef {
                name: "tokio".to_string(),

                source: DDependencyDefSource::Tag {
                    tag: "v1.2.3.4".to_string(),
                },
            }, "tokio:tag=v1.2.3.4");
        }
    }

    mod when_given_source_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_attachment_source() {
            assert(DDependencyDef {
                name: "tokio".to_string(),

                source: DDependencyDefSource::Tag {
                    tag: "v1.2.3.4".to_string(),
                },
            }, "tokio:source=../my-tokio");
        }
    }

    mod when_given_version_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_version_source() {
            assert(DDependencyDef {
                name: "tokio".to_string(),

                source: DDependencyDefSource::Version {
                    version: "0.2.1-alpha".to_string(),
                },
            }, "tokio=0.2.1-alpha");
        }
    }

    fn assert(expected: DDependencyDef, actual: &str) {
        let actual = parse_dependency(actual)
            .unwrap();

        assert_eq!(expected, actual);
    }
}