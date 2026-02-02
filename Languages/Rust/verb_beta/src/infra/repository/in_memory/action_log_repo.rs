use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::application::ApplicationError;
use crate::domain::{
    model::{ActionLog, VerbId},
    repository::ActionLogRepository,
};

/// In-memory action log repository
#[derive(Clone)]
pub struct InMemoryActionLogRepo {
    store: Arc<Mutex<Vec<ActionLog>>>,
}

impl InMemoryActionLogRepo {
    pub fn new(store: Arc<Mutex<Vec<ActionLog>>>) -> Self {
        Self { store }
    }
}

impl ActionLogRepository for InMemoryActionLogRepo {
    fn append(
        &self,
        log: &ActionLog,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        let log = log.clone();
        let store = Arc::clone(&self.store);

        Box::pin(async move {
            let mut guard = store.lock().await;
            guard.push(log);
            Ok(())
        })
    }

    fn find_by_verb(
        &self,
        verb_id: VerbId,
        limit: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ActionLog>, ApplicationError>> + Send + '_>> {
        let store = Arc::clone(&self.store);

        Box::pin(async move {
            let guard = store.lock().await;

            let mut logs: Vec<ActionLog> = guard
                .iter()
                .filter(|log| log.verb_id() == verb_id)
                .cloned()
                .collect();

            // Sort by timestamp desc
            logs.sort_by(|a, b| b.timestamp().cmp(&a.timestamp()));

            // Limit
            logs.truncate(limit as usize);

            Ok(logs)
        })
    }
}
