use anyhow::*;

use lib_compiler::Ecosystem;
use lib_interop::models::{DDefinition, DJob};

pub struct Compiler {
    compiler: lib_compiler::Compiler,
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Result<Self> {
        let mut compiler = lib_compiler::Compiler::builder();

        compiler.set_ecosystem(ecosystem);

        Ok(Self {
            compiler: compiler.build()?,
        })
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        self.compiler.compile(definition)
    }
}
