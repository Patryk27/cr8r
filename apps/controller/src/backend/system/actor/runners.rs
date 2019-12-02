use std::collections::HashMap;

use bimap::BiMap;

use lib_protocol::core::{RunnerId, RunnerName};

use crate::backend::{Result, Runner, System};
use crate::id;

pub struct Runners {
    system: System,
    index: BiMap<RunnerId, RunnerName>,
    runners: HashMap<RunnerId, Runner>,
}

impl Runners {
    pub fn new(system: System) -> Self {
        Self {
            system,
            index: BiMap::new(),
            runners: HashMap::new(),
        }
    }

    pub fn create(&mut self, name: RunnerName) -> Result<RunnerId> {
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

    pub fn remove(&mut self, id: RunnerId) -> bool {
        self.index
            .remove_by_left(&id)
            .is_some()
    }

    pub fn all(&self) -> Vec<Runner> {
        self.runners
            .values()
            .map(ToOwned::to_owned)
            .collect()
    }
}