//! @todo error messages yielded while parsing dependencies are hella cryptic

use std::str;

use anyhow::*;
use nom::character::complete::char;
use nom::sequence::{terminated, tuple};

pub use self::{
    name::*,
    source::*,
};

mod name;
mod source;

#[derive(Debug, PartialEq)]
pub struct DependencyArg {
    pub name: DependencyNameArg,
    pub source: DependencySourceArg,
}

impl str::FromStr for DependencyArg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let dep_name = terminated(DependencyNameArg::parse, char(':'));
        let dep_source = DependencySourceArg::parse;

        let dep = tuple((
            dep_name,
            dep_source,
        ))(s);

        let (_, (name, source)) = dep.map_err(|_| anyhow!("Unknown syntax"))?;

        Ok(Self {
            name,
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    mod when_given_branch_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_branch_source() {
            assert(DependencyArg {
                name: DependencyNameArg("tokio".to_string()),
                source: DependencySourceArg::Branch("features/performance-fix".to_string()),
            }, "tokio:branch=features/performance-fix");
        }
    }

    mod when_given_tag_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_tag_source() {
            assert(DependencyArg {
                name: DependencyNameArg("tokio".to_string()),
                source: DependencySourceArg::Tag("v1.2.3.4".to_string()),
            }, "tokio:tag=v1.2.3.4");
        }
    }

    mod when_given_version_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_version_source() {
            assert(DependencyArg {
                name: DependencyNameArg("tokio".to_string()),
                source: DependencySourceArg::Version("0.2.1-alpha".to_string()),
            }, "tokio:version=0.2.1-alpha");
        }
    }

    mod when_given_path_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_path_source() {
            assert(DependencyArg {
                name: DependencyNameArg("tokio".to_string()),
                source: DependencySourceArg::Path("../tokio".to_string()),
            }, "tokio:path=../tokio");
        }
    }

    fn assert(expected: DependencyArg, actual: &str) {
        let actual = DependencyArg::from_str(actual)
            .unwrap();

        assert_eq!(expected, actual);
    }
}
