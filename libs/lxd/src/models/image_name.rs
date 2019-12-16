use std::fmt;
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug)]
pub struct LxdImageName(String);

impl LxdImageName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for LxdImageName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.to_string()
        ))
    }
}

impl fmt::Display for LxdImageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}