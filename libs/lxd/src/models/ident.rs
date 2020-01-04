use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LxdIdent(String);

impl LxdIdent {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for LxdIdent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // @todo validate for [a-zA-Z0-9_]+

        Ok(Self(
            s.to_string()
        ))
    }
}

impl fmt::Display for LxdIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[macro_export]
macro_rules! newtype {
    ($name:ident) => {
        use anyhow::{Error, Result};
        use serde::Deserialize;
        use std::fmt;
        use std::str::FromStr;

        use crate::LxdIdent;

        #[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
        pub struct $name(LxdIdent);

        impl $name {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                Ok(Self(s.parse()?))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}