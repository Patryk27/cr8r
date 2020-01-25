use std::{fmt, str};

use anyhow::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LxdIdent(String);

impl LxdIdent {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LxdIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for LxdIdent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // @todo validate for [a-zA-Z0-9_]+

        Ok(Self(
            s.to_string()
        ))
    }
}

// @todo deserialize should validate whether the identifier is actually valid
#[macro_export]
macro_rules! newtype {
    ($name:ident) => {
        use anyhow::*;
        use serde::Deserialize;
        use std::{fmt, str};

        use crate::LxdIdent;

        #[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
        pub struct $name(LxdIdent);

        impl $name {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                Ok(Self(s.parse()?))
            }
        }
    }
}