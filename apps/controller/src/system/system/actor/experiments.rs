use std::collections::{HashMap, VecDeque};

use lib_protocol::core::{experiment_definition::ExperimentDefinitionInner, Scenario};

use crate::system::{ExperimentId, RunnerId};

pub struct Experiments {
    definitions: HashMap<ExperimentId, ExperimentDefinitionInner>,
    scenarios: HashMap<ExperimentId, Vec<Scenario>>,
    pending: VecDeque<ExperimentId>,
    unclaimed: VecDeque<ExperimentId>,
}

impl Experiments {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            scenarios: HashMap::new(),
            pending: VecDeque::new(),
            unclaimed: VecDeque::new(),
        }
    }

    pub fn create(&mut self, definition: ExperimentDefinitionInner, scenarios: Vec<Scenario>) -> ExperimentId {
        let id = ExperimentId::new_v4();

        self.definitions.insert(id, definition);
        self.scenarios.insert(id, scenarios);
        self.pending.push_back(id);

        id
    }

    pub fn claim(&mut self, experiment: ExperimentId, runner: RunnerId) {
        unimplemented!()
    }

    pub fn take_pending(&mut self) -> Option<ExperimentId> {
        let id = self.pending.pop_back()?;
        self.unclaimed.push_back(id);
        Some(id)
    }
}