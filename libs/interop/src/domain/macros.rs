#[macro_export]
macro_rules! newtype {
    ($name:ident: Uuid) => {
        $crate::newtype! {
            @gen $name
        }

        impl $name {
            pub fn default() -> Self {
                uuid::Uuid::new_v4()
                    .to_hyphenated()
                    .to_string()
                    .into()
            }
        }
    };

    ($name:ident: String) => {
        $crate::newtype! {
            @gen $name
        }
    };

    (@gen $name:ident) => {
        use std::fmt;

        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(str: String) -> Self {
                Self(str)
            }
        }

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! convert {
    ($field:ident? $($tt:tt)*) => {{
        use $crate::Error;

        let field = $field.ok_or_else(|| {
            Error::Missing {
                field: stringify!($field),
            }
        })?;

        convert!(field $($tt)*)
    }};

    // Having `convert!(x)` as a no-op is useful, because it nicely adapts with other conversion routines, e.g.
    // `convert!(x?)`
    ($field:ident) => {
        $field
    };

    ($field:ident as _) => {
        $field.into()
    };

    ($field:ident as _?) => {{
        use std::convert::TryInto;

        $field.try_into()?
    }};

    ($field:ident as [_]) => {
        $field
            .into_iter()
            .map(Into::into)
            .collect()
    };

    ($field:ident as [_?]) => {{
        use $crate::Result;
        use std::convert::TryInto;

        let field = $field
            .into_iter()
            .map(TryInto::try_into)
            .collect(): Result<Vec<_>>;

        field?
    }};

    ($field:ident as DateTime) => {{
        use $crate::error;
        use chrono::{DateTime, Utc};
        use snafu::ResultExt;

        DateTime::parse_from_rfc3339(&$field)
            .context(error::InvalidDateTime { field: stringify!($field) })?
            .with_timezone(&Utc)
    }};
}