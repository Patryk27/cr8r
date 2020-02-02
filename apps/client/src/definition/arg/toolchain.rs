use std::str;

use anyhow::*;

#[derive(Debug)]
pub struct ToolchainArg(String);

impl str::FromStr for ToolchainArg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.to_string()))
    }
}