use std::collections::{HashMap, VecDeque};

use lib_protocol::core::{ExperimentId, Scenario};

use crate::backend::{Experiment, Result, System};
use crate::id;

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
        let id = id!();

        let experiment = Experiment::spawn(
            self.system.clone(),
            id.clone(),
            scenarios,
        );

        self.experiments.insert(id.clone(), experiment);
        self.pending.push_back(id.clone());

        id
    }

    pub fn take(&mut self) -> Option<Experiment> {
        let id = self.pending.pop_back()?;

        let experiment = self.experiments
            .get(&id)
            .expect("System's state is inconsistent: an experiment is missing inside the `experiments` map")
            .to_owned();

        Some(experiment)
    }

    pub fn get(&self, id: &ExperimentId) -> Result<Experiment> {
        self.experiments
            .get(id)
            .map(ToOwned::to_owned)
            .ok_or_else(|| format!("Experiment `{}` does not exist", id).into())
    }

    pub fn all(&self) -> Vec<Experiment> {
        self.experiments
            .values()
            .map(ToOwned::to_owned)
            .collect()
    }
}