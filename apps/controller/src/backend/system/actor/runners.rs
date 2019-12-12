use std::collections::HashMap;

use bimap::BiMap;

use lib_protocol::core::{PRunnerId, PRunnerName};

use crate::backend::{Result, Runner, System};
use crate::id;

pub struct Runners {
    system: System,
    index: BiMap<PRunnerId, PRunnerName>,
    runners: HashMap<PRunnerId, Runner>,
}

impl Runners {
    pub fn new(system: System) -> Self {
        Self {
            system,
            index: BiMap::new(),
            runners: HashMap::new(),
        }
    }

    pub fn create(&mut self, name: PRunnerName) -> Result<PRunnerId> {
        if self.index.contains_right(&name) {
            return Err("Runner with such name has been already registered".into());
        }

        let id = id!();

        let runner = Runner::spawn(
            self.system.clone(),
            id.clone(),
            name.clone(),
        );

        self.index.insert(id.clone(), name);
        self.runners.insert(id.clone(), runner);

        Ok(id)
    }

    pub fn remove(&mut self, id: &PRunnerId) {
        self.index.remove_by_left(id);
        self.runners.remove(id);
    }

    pub fn get(&self, id: &PRunnerId) -> Option<&Runner> {
        self.runners.get(id)
    }

    pub fn all(&self) -> Vec<&Runner> {
        self.runners
            .values()
            .collect()
    }

    pub fn name_to_id(&self, name: &PRunnerName) -> Option<&PRunnerId> {
        self.index.get_by_right(name)
    }

    pub fn validate(&self, id: &PRunnerId) -> Result<()> {
        self.index
            .get_by_left(id)
            .map(|_| ())
            .ok_or_else(|| "No such runner exists".into())
    }
}