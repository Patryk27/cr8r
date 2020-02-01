use std::collections::{HashMap, VecDeque};

use anyhow::*;

use lib_interop::domain::{DExperimentId, DJob};

use crate::backend::Experiment;

pub struct Experiments {
    experiments: HashMap<DExperimentId, Experiment>,
    pending: VecDeque<DExperimentId>,
    next_id: DExperimentId,
}

impl Experiments {
    pub fn new() -> Self {
        Self {
            experiments: HashMap::new(),
            pending: VecDeque::new(),
            next_id: DExperimentId::default(),
        }
    }

    pub fn create(&mut self, jobs: Vec<DJob>) -> DExperimentId {
        let id = self.next_id.inc();

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
            .ok_or_else(|| anyhow!("Experiment `{}` does not exist", id))
    }

    pub fn all(&self) -> Vec<Experiment> {
        self.experiments
            .values()
            .map(ToOwned::to_owned)
            .collect()
    }
}