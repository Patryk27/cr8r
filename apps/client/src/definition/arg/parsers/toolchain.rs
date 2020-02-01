use lib_interop::domain::definition::definition_inner::DToolchainDef;

pub fn parse_toolchain(toolchain: Option<String>) -> Option<DToolchainDef> {
    toolchain.map(|toolchain| {
        DToolchainDef { toolchain }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_given_unspecified_toolchain {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn then_returns_none() {
            assert_eq!(
                None,
                parse_toolchain(None),
            );
        }
    }

    mod when_given_specified_toolchain {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn then_returns_that_toolchain() {
            let actual = parse_toolchain(
                Some("nightly".to_string()),
            );

            let expected = Some(DToolchainDef {
                toolchain: "nightly".to_string(),
            });

            assert_eq!(expected, actual);
        }
    }
}