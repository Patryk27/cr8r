#[macro_export]
macro_rules! create_identifier_model {
    ($name:ident: uuid) => {
        crate::create_identifier_model! {
            @gen $name
        }

        impl $name {
            pub fn new() -> Self {
                uuid::Uuid::new_v4()
                    .to_hyphenated()
                    .to_string()
                    .into()
            }
        }
    };

    ($name:ident: string) => {
        crate::create_identifier_model! {
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