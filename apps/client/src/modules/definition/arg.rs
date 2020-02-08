use structopt::StructOpt;

pub use self::{
    dependency::*,
    toolchain::*,
};

mod dependency;
mod toolchain;

#[derive(Debug, StructOpt)]
pub struct DefinitionArg {
    #[structopt(long = "toolchain", short = "t")]
    pub toolchain: Option<ToolchainArg>,

    #[structopt(long = "dependency", short = "d")]
    pub dependencies: Vec<DependencyArg>,
}

impl DefinitionArg {
    pub fn contains_path_deps(&self) -> bool {
        self.dependencies
            .iter()
            .any(|dep| dep.source.is_path())
    }
}