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

        // @todo this should probably be `TryFrom<String>` for cases like `str.is_empty()`
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
        use $crate::domain::DomainError;

        let field = $field
            .ok_or_else(|| DomainError::MissingField { name: stringify!($field) })?;

        convert!(field $($tt)*)
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
        use $crate::domain::DomainResult;
        use std::convert::TryInto;

        let field = $field
            .into_iter()
            .map(TryInto::try_into)
            .collect(): DomainResult<Vec<_>>;
                                    // we're always collecting into a `Vec`, so that the entire macro is more convenient
                                    // to use; you can try getting rid of that `Vec` and see what happens

        field?
    }};

    ($field:ident as { _ => _ }) => {{
        $field
            .into_iter()
            .map(|(key, value)| {
                let key = convert!(key as _);
                let value = convert!(value as _);

                (key, value)
            })
            .collect()
    }};

    ($field:ident as { _ => _? }) => {{
        use $crate::domain::DomainResult;

        let fields = $field
            .into_iter()
            .map(|(key, value)| {
                let key = convert!(key as _);
                let value = convert!(value as _?);

                Ok((key, value))
            })
            .collect(): DomainResult<_>;

        fields?
    }};

    ($field:ident as DateTime) => {{
        use $crate::domain::DomainError;
        use chrono::{DateTime, Utc};

        DateTime::parse_from_rfc3339(&$field)
            .map_err(|source| DomainError::InvalidField { name: stringify!($field), source: source.into() })?
            .with_timezone(&Utc)
    }};
}
