use crate::services::core::model::action_log::ActionLog;
use crate::services::core::{error::CoreError, store::verb_store::ActionLogStore};

use std::sync::{Arc, Mutex};

use super::uow::MemoryDb;

#[derive(Clone)]
pub struct MemoryActionLogStore {
    pub db: Arc<Mutex<MemoryDb>>,
}

impl ActionLogStore for MemoryActionLogStore {
    fn log(&self, action: &ActionLog) -> Result<(), CoreError> {
        let mut db = self.db.lock().unwrap();
        db.logs.push(action.clone());
        Ok(())
    }
}
