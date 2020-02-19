#[macro_export]
macro_rules! newtype {
    ($name:ident as number) => {
        use std::fmt;

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(u32);

        impl $name {
            pub fn as_num(&self) -> u32 {
                self.0
            }

            pub fn inc(&mut self) -> Self {
                let this = *self;
                self.0 += 1;
                this
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(1)
            }
        }

        // @todo this should probably be `TryFrom<u32>` for the `num == 0` case
        impl From<u32> for $name {
            fn from(num: u32) -> Self {
                Self(num)
            }
        }

        impl Into<u32> for $name {
            fn into(self) -> u32 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };

    ($name:ident as string) => {
        use std::fmt;

        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(str: &str) -> Self {
                str.to_string().into()
            }
        }

        // @todo this should probably be `TryFrom<String>` for the `str.is_empty()` case
        impl From<String> for $name {
            fn from(str: String) -> Self {
                Self(str)
            }
        }

        impl From<&String> for $name {
            fn from(str: &String) -> Self {
                Self(str.to_owned())
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
macro_rules! conv {
    ($field:ident? $($tt:tt)*) => {{
        use $crate::models::ModelError;

        let field = $field
            .ok_or_else(|| ModelError::MissingField { name: stringify!($field) })?;

        conv!(field $($tt)*)
    }};

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
        use $crate::models::ModelResult;
        use std::convert::TryInto;

        let field = $field
            .into_iter()
            .map(TryInto::try_into)
            .collect(): ModelResult<Vec<_>>;
                                    // we're always collecting into a `Vec`, so that the entire macro is more convenient
                                    // to use; you can try getting rid of that `Vec` and see what happens for yourself

        field?
    }};

    ($field:ident as { _ => _ }) => {{
        $field
            .into_iter()
            .map(|(key, value)| {
                let key = conv!(key as _);
                let value = conv!(value as _);

                (key, value)
            })
            .collect()
    }};

    ($field:ident as { _ => _? }) => {{
        use $crate::models::ModelResult;

        let fields = $field
            .into_iter()
            .map(|(key, value)| {
                let key = conv!(key as _);
                let value = conv!(value as _?);

                Ok((key, value))
            })
            .collect(): ModelResult<_>;

        fields?
    }};

    ($field:ident as DateTime) => {{
        use $crate::models::ModelError;
        use chrono::{DateTime, Utc};

        DateTime::parse_from_rfc3339(&$field)
            .map_err(|source| ModelError::InvalidField { name: stringify!($field), source: source.into() })?
            .with_timezone(&Utc)
    }};
}
