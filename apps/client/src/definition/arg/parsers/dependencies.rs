use anyhow::*;

use lib_interop::domain::definition_inner::DDependencyDef;

use super::parse_dependency;

pub fn parse_dependencies(deps: Vec<String>) -> Result<Vec<DDependencyDef>> {
    // @todo find duplicates

    deps.into_iter()
        .map(|dep| {
            parse_dependency(&dep)
                .with_context(|| format!("Could not understand dependency `{}`", dep))
        })
        .collect()
}
