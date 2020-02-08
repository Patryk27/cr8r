use std::collections::HashMap;

use anyhow::*;
use bimap::BiMap;

use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::backend::{Runner, System};

pub struct Runners {
    system: System,
    index: BiMap<DRunnerId, DRunnerName>,
    runners: HashMap<DRunnerId, Runner>,
    next_id: DRunnerId,
}

impl Runners {
    pub fn new(system: System) -> Self {
        Self {
            system,
            index: BiMap::new(),
            runners: HashMap::new(),
            next_id: DRunnerId::default(),
        }
    }

    pub fn create(&mut self, name: DRunnerName) -> Result<DRunnerId> {
        if self.index.contains_right(&name) {
            return Err(anyhow!("Runner with this name already exists"));
        }

        let id = self.next_id.inc();

        let runner = Runner::new(
            self.system.clone(),
            id.clone(),
            name.clone(),
        );

        self.index.insert(id.clone(), name);
        self.runners.insert(id.clone(), runner);

        Ok(id)
    }

    pub fn remove(&mut self, id: &DRunnerId) {
        self.index.remove_by_left(id);
        self.runners.remove(id);
    }

    pub fn get(&self, id: &DRunnerId) -> Option<&Runner> {
        self.runners.get(id)
    }

    pub fn all(&self) -> Vec<&Runner> {
        self.runners
            .values()
            .collect()
    }

    pub fn name_to_id(&self, name: &DRunnerName) -> Option<&DRunnerId> {
        self.index.get_by_right(name)
    }

    /* pub fn validate(&self, id: &DRunnerId) -> Result<()> {
        self.index
            .get_by_left(id)
            .map(|_| ())
            .ok_or_else(|| anyhow!("No such runner exists"))
    } @todo */
}