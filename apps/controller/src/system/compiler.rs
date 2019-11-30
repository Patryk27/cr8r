use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;
use lib_protocol::core::Scenario;

use crate::Ecosystem;

pub struct Compiler {
    ecosystem: Ecosystem,
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Self {
        Self { ecosystem }
    }

    pub fn compile(&self, definition: &ExperimentDefinitionInner) -> Vec<Scenario> {
        unimplemented!()
    }
}