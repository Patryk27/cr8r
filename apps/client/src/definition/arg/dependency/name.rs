use nom::bytes::complete::take_while;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct DependencyNameArg(pub String);

impl DependencyNameArg {
    pub(super) fn parse(s: &str) -> IResult<&str, Self> {
        let name = take_while(|c: char| {
            c.is_alphanumeric()
                || c == '-'
                || c == '_'
        });

        map(name, |name: &str| DependencyNameArg(name.to_string()))(s)
    }
}
