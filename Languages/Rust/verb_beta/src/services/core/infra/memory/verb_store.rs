use crate::services::core::error::CoreError;
use crate::services::core::model::{VerbId, verb::Verb};
use crate::services::core::store::verb_store::VerbStore;
use uuid::Uuid;

use std::sync::{Arc, Mutex};

use super::uow::MemoryDb;

#[derive(Clone)]
pub struct MemoryVerbStore {
    pub db: Arc<Mutex<MemoryDb>>,
}

impl VerbStore for MemoryVerbStore {
    fn save(&self, verb: &Verb) -> Result<(), CoreError> {
        let mut db = self.db.lock().unwrap();
        db.verbs.push(verb.clone());
        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Verb>, CoreError> {
        let db = self.db.lock().unwrap();
        Ok(db
            .verbs
            .iter()
            .cloned()
            .find(|v| v.id() == VerbId::from_uuid(id)))
    }
}
