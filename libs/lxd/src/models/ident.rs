use std::convert::TryFrom;
use std::fmt;

use serde::Deserialize;

use crate::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LxdIdent(String);

impl LxdIdent {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for LxdIdent {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // @todo validate for [a-zA-Z0-9_]+
        Ok(Self(value))
    }
}

impl fmt::Display for LxdIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! create_ident_type {
    ($ty:ident) => {
        use serde::Deserialize;
        use std::convert::TryFrom;
        use std::fmt;

        use crate::{Error, LxdIdent};

        #[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
        pub struct $ty(LxdIdent);

        impl $ty {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl TryFrom<String> for $ty {
            type Error = Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                LxdIdent::try_from(value).map(Self)
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}