//! @todo error messages yielded while parsing dependencies are hella cryptic

use anyhow::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::char;
use nom::combinator::{map, rest};
use nom::IResult;
use nom::sequence::{preceded, terminated, tuple};

use lib_interop::domain::definition_inner::{DDependencyDef, DDependencyDefSource};

pub fn parse_dependency(dep: &str) -> Result<DDependencyDef> {
    let dep = tuple((
        terminated(dep_name, char(':')),
        dep_source,
    ))(dep);

    let (_, (name, source)) = dep.map_err(|err| {
        anyhow!("Dependency contains syntax error; for reference, the parser returned: {:?}", err)
    })?;

    Ok(DDependencyDef {
        name: name.to_string(),
        source,
    })
}

fn dep_name(dep: &str) -> IResult<&str, &str> {
    take_while(|c: char| {
        c.is_alphanumeric()
            || c == '-'
            || c == '_'
    })(dep)
}

fn dep_source(dep: &str) -> IResult<&str, DDependencyDefSource> {
    let branch_src = preceded(
        tag("branch="),
        rest,
    );

    let tag_src = preceded(
        tag("tag="),
        rest,
    );

    let version_src = preceded(
        tag("version="),
        rest,
    );

    alt((
        map(branch_src, |branch: &str| DDependencyDefSource::Branch {
            branch: branch.to_string(),
        }),
        // --- //
        map(tag_src, |tag: &str| DDependencyDefSource::Tag {
            tag: tag.to_string(),
        }),
        // --- //
        map(version_src, |version: &str| DDependencyDefSource::Version {
            version: version.to_string(),
        })
    ))(dep)
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

    mod when_given_version_override {
        use super::*;

        #[test]
        fn then_returns_dependency_with_version_source() {
            assert(DDependencyDef {
                name: "tokio".to_string(),

                source: DDependencyDefSource::Version {
                    version: "0.2.1-alpha".to_string(),
                },
            }, "tokio:version=0.2.1-alpha");
        }
    }

    // @todo
//    mod when_given_source_override {
//        use super::*;
//
//        #[test]
//        fn then_returns_dependency_with_patch_source() {
//            assert(DDependencyDef {
//                name: "tokio".to_string(),
//
//                source: DDependencyDefSource::Patch {
//                    attachment_id: unimplemented!(),
//                },
//            }, "tokio:source=../my-tokio");
//        }
//    }

    fn assert(expected: DDependencyDef, actual: &str) {
        let actual = parse_dependency(actual)
            .unwrap();

        assert_eq!(expected, actual);
    }
}