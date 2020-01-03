use std::collections::{HashMap, VecDeque};

use lib_interop::domain::{DExperimentId, DJob};

use crate::backend::{Experiment, Result};

pub struct Experiments {
    experiments: HashMap<DExperimentId, Experiment>,
    pending: VecDeque<DExperimentId>,
}

impl Experiments {
    pub fn new() -> Self {
        Self {
            experiments: HashMap::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn create(&mut self, jobs: Vec<DJob>) -> DExperimentId {
        let id = DExperimentId::default();

        let experiment = Experiment::new(
            id.clone(),
            jobs,
        );

        self.experiments.insert(id.clone(), experiment);
        self.pending.push_back(id.clone());

        id
    }

    pub fn pop(&mut self) -> Option<Experiment> {
        let id = self.pending.pop_back()?;

        let experiment = self.experiments
            .get(&id)
            .expect("System's state is inconsistent: an experiment is missing inside the `experiments` map")
            .to_owned();

        Some(experiment)
    }

    pub fn get(&self, id: &DExperimentId) -> Result<Experiment> {
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