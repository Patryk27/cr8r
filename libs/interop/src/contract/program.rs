pub use self::step::*;

mod step;

#[derive(Clone, Debug)]
pub struct CProgram {
    system: String,
    toolchain: String,
    steps: Vec<CProgramStep>,
}

impl CProgram {
    pub fn new(system: String, toolchain: String) -> Self {
        Self {
            system,
            toolchain,
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: CProgramStep) {
        self.steps.push(step);
    }

    pub fn steps(&self) -> &Vec<CProgramStep> {
        &self.steps
    }
}