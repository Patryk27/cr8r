use std::collections::HashMap;

use bimap::BiMap;

use lib_interop::contract::{CRunnerId, CRunnerName};

use crate::backend::{Result, Runner, System};

pub struct Runners {
    system: System,
    index: BiMap<CRunnerId, CRunnerName>,
    runners: HashMap<CRunnerId, Runner>,
}

impl Runners {
    pub fn new(system: System) -> Self {
        Self {
            system,
            index: BiMap::new(),
            runners: HashMap::new(),
        }
    }

    pub fn create(&mut self, name: CRunnerName) -> Result<CRunnerId> {
        if self.index.contains_right(&name) {
            return Err("Runner with such name has been already registered".into());
        }

        let id = CRunnerId::new();

        let runner = Runner::new(
            self.system.clone(),
            id.clone(),
            name.clone(),
        );

        self.index.insert(id.clone(), name);
        self.runners.insert(id.clone(), runner);

        Ok(id)
    }

    pub fn remove(&mut self, id: &CRunnerId) {
        self.index.remove_by_left(id);
        self.runners.remove(id);
    }

    pub fn get(&self, id: &CRunnerId) -> Option<&Runner> {
        self.runners.get(id)
    }

    pub fn all(&self) -> Vec<&Runner> {
        self.runners
            .values()
            .collect()
    }

    pub fn name_to_id(&self, name: &CRunnerName) -> Option<&CRunnerId> {
        self.index.get_by_right(name)
    }

    pub fn validate(&self, id: &CRunnerId) -> Result<()> {
        self.index
            .get_by_left(id)
            .map(|_| ())
            .ok_or("No such runner exists".into())
    }
}