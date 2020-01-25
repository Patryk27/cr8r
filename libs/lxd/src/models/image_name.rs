use std::{fmt, str};

use anyhow::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LxdImageName(String);

impl LxdImageName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LxdImageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for LxdImageName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.to_string()
        ))
    }
}
