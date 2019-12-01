use std::collections::{HashMap, VecDeque};

use lib_protocol::core::{ExperimentId, Scenario};

use crate::backend::{Experiment, Result, System, uuid};

pub struct Experiments {
    system: System,
    experiments: HashMap<ExperimentId, Experiment>,
    pending: VecDeque<ExperimentId>,
}

impl Experiments {
    pub fn new(system: System) -> Self {
        Self {
            system,
            experiments: HashMap::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn create(&mut self, scenarios: Vec<Scenario>) -> ExperimentId {
        let id = uuid!();

        let experiment = Experiment::spawn(
            self.system.clone(),
            id.clone(),
            scenarios,
        );

        self.experiments.insert(id.clone(), experiment);
        self.pending.push_back(id.clone());

        id
    }

    pub fn find_by_id(&mut self, id: &ExperimentId) -> Result<&Experiment> {
        self.experiments
            .get(id)
            .ok_or_else(|| format!("Experiment `{}` does not exist", id).into())
    }

    pub fn take(&mut self) -> Option<Experiment> {
        let id = self.pending.pop_back()?;

        let experiment = self.experiments
            .get(&id)
            .expect("System's state is inconsistent: an experiment is missing inside the `experiments` map")
            .to_owned();

        Some(experiment)
    }
}