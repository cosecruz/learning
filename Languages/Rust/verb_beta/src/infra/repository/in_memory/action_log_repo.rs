use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::application::ApplicationError;
use crate::domain::repository::action_log_repo::ActionLogFilter;
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

    ///Use find by verb with filters
    fn find_by_verb(
        &self,
        verb_id: VerbId,
        filter: &ActionLogFilter,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ActionLog>, ApplicationError>> + Send + '_>> {
        let store = Arc::clone(&self.store);
        let filter = filter.clone();
        Box::pin(async move {
            let guard = store.lock().await;

            let mut logs: Vec<ActionLog> = guard
                .iter()
                .filter(|log| log.verb_id() == verb_id)
                .filter(|log| {
                    if let Some(state) = filter.state {
                        log.action_type() == state
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            drop(guard); // release lock early (important for async systems)

            // Sort by timestamp desc
            logs.sort_by_key(|b| std::cmp::Reverse(b.timestamp()));

            // Limit
            // logs.truncate(filter.limit as usize);
            // Step 3: pagination
            let offset = filter.offset as usize;
            let limit = filter.limit as usize;

            let logs = logs.into_iter().skip(offset).take(limit).collect();

            Ok(logs)
        })
    }
}
