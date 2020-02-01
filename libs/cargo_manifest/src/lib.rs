use std::str::FromStr;

use toml::value::Table;

pub use self::{
    dependencies::*,
    error::*,
};

mod dependencies;
mod error;

#[derive(Debug)]
pub struct CargoManifest {
    inner: Table,
}

impl CargoManifest {
    pub fn print(&self) -> Result<String> {
        Ok(toml::to_string_pretty(&self.inner)?)
    }
}

impl FromStr for CargoManifest {
    type Err = CargoManifestError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            inner: toml::from_str(s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_no_changes_are_applied {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn then_manifest_remains_the_same() {
            let editor = CargoManifest::from_str(r#"
                [package]
                name = 'hello-world'
                version = '0.1.0'

                [dependencies]
                foo = '0.1'
            "#).unwrap();

            assert_eq!(
                "[package]\n\
                 name = 'hello-world'\n\
                 version = '0.1.0'\n\
                 \n\
                 [dependencies]\n\
                 foo = '0.1'\n",
                editor.print().unwrap(),
            );
        }
    }
}