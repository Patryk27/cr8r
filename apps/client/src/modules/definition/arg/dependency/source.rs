use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, rest};
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, PartialEq)]
pub enum DependencySourceArg {
    Branch(String),
    Tag(String),
    Version(String),
    Path(String),
}

impl DependencySourceArg {
    pub fn is_path(&self) -> bool {
        if let DependencySourceArg::Path(_) = &self {
            true
        } else {
            false
        }
    }

    pub(super) fn parse(s: &str) -> IResult<&str, Self> {
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

        let path_src = preceded(
            tag("path="),
            rest,
        );

        alt((
            map(branch_src, |branch: &str| {
                DependencySourceArg::Branch(
                    branch.to_string(),
                )
            }),

            // -- //

            map(tag_src, |tag: &str| {
                DependencySourceArg::Tag(
                    tag.to_string(),
                )
            }),

            // -- //

            map(version_src, |version: &str| {
                DependencySourceArg::Version(
                    version.to_string(),
                )
            }),

            // -- //

            map(path_src, |path: &str| {
                DependencySourceArg::Path(
                    path.to_string(),
                )
            }),
        ))(s)
    }
}