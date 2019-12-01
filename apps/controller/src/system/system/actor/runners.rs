use std::collections::HashMap;

use bimap::BiMap;

use crate::system::{Result, RunnerId, RunnerName, RunnerSession};

pub struct Runners {
    index: BiMap<RunnerId, RunnerName>,
    sessions: HashMap<RunnerId, RunnerSession>,
}

impl Runners {
    pub fn new() -> Self {
        Self {
            index: BiMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: RunnerName) -> Result<RunnerId> {
        if self.index.contains_right(&name) {
            return Err("Runner with such name has been already registered".to_string());
        }

        unimplemented!()
//
//        let id = RunnerId::new_v4();
//        let session = RunnerSession::start();
//
//        Ok(token)
    }

    pub fn unregister(&mut self, id: RunnerId) -> bool {
        self.index
            .remove_by_left(&id)
            .is_some()
    }
}