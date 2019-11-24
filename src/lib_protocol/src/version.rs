use semver::Version;

pub fn version() -> Version {
    Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: Vec::default(),
        build: Vec::default(),
    }
}

pub fn is_compatible_with(other: &Version) -> bool {
    // @todo this could be improved
    other == &version()
}