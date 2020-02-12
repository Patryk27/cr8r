use std::{fmt, str};

use anyhow::*;

#[derive(Debug)]
pub struct ToolchainArg(pub String);

impl fmt::Display for ToolchainArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for ToolchainArg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.to_string()))
    }
}