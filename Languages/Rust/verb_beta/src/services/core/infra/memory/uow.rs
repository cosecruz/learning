use crate::services::core::{
    error::CoreError,
    store::verb_store::{Transaction, UnitOfWork},
};

use std::sync::{Arc, Mutex};

/// In-memory transactional buffer
#[derive(Default)]
pub struct MemoryDb {
    pub verbs: Vec<crate::services::core::model::verb::Verb>,
    pub logs: Vec<crate::services::core::model::action_log::ActionLog>,
}

#[derive(Clone)]
pub struct MemoryUow {
    pub db: Arc<Mutex<MemoryDb>>,
}

pub struct MemoryTx {
    db: Arc<Mutex<MemoryDb>>,
    staged_verbs: Vec<crate::services::core::model::verb::Verb>,
    staged_logs: Vec<crate::services::core::model::action_log::ActionLog>,
}

impl UnitOfWork for MemoryUow {
    fn begin(&self) -> Result<Box<dyn Transaction>, CoreError> {
        Ok(Box::new(MemoryTx {
            db: self.db.clone(),
            staged_verbs: vec![],
            staged_logs: vec![],
        }))
    }
}

impl MemoryTx {
    pub fn stage_verb(&mut self, verb: crate::services::core::model::verb::Verb) {
        self.staged_verbs.push(verb);
    }

    pub fn stage_log(&mut self, log: crate::services::core::model::action_log::ActionLog) {
        self.staged_logs.push(log);
    }
}

impl Transaction for MemoryTx {
    fn commit(self: Box<Self>) -> Result<(), CoreError> {
        let mut db = self.db.lock().unwrap();

        db.verbs.extend(self.staged_verbs);
        db.logs.extend(self.staged_logs);

        Ok(())
    }

    fn rollback(self: Box<Self>) -> Result<(), CoreError> {
        // Just drop staged data
        Ok(())
    }
}
