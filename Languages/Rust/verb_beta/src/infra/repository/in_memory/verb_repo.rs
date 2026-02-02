use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::application::ApplicationError;
use crate::domain::repository::verb_repo::{VerbFilter, VerbListResult};
use crate::domain::{
    model::{Verb, VerbId},
    repository::VerbRepository,
};

/// In-memory verb repository
///
/// Implements VerbRepository trait using an in-memory Vec.
#[derive(Clone)]
pub struct InMemoryVerbRepo {
    store: Arc<Mutex<Vec<Verb>>>,
}

impl InMemoryVerbRepo {
    pub fn new(store: Arc<Mutex<Vec<Verb>>>) -> Self {
        Self { store }
    }
}

impl VerbRepository for InMemoryVerbRepo {
    fn save(
        &self,
        verb: &Verb,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>> {
        let verb = verb.clone();
        let store = Arc::clone(&self.store);

        Box::pin(async move {
            let mut guard = store.lock().await;

            // Update if exists, insert if new
            if let Some(existing) = guard.iter_mut().find(|v| v.id() == verb.id()) {
                *existing = verb;
            } else {
                guard.push(verb);
            }

            Ok(())
        })
    }

    fn find_by_id(
        &self,
        id: VerbId,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Verb>, ApplicationError>> + Send + '_>> {
        let store = Arc::clone(&self.store);

        Box::pin(async move {
            let guard = store.lock().await;
            Ok(guard.iter().find(|v| v.id() == id).cloned())
        })
    }

    fn list(
        &self,
        filter: VerbFilter,
    ) -> Pin<Box<dyn Future<Output = Result<VerbListResult, ApplicationError>> + Send + '_>> {
        let store = Arc::clone(&self.store);

        Box::pin(async move {
            let guard = store.lock().await;

            let mut verbs: Vec<Verb> = guard
                .iter()
                .filter(|v| {
                    if let Some(state) = filter.state {
                        v.state() == state
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            // Sort by updated_at desc
            verbs.sort_by(|a, b| b.updated_at().cmp(&a.updated_at()));

            // total BEFORE pagination
            let total: u32 = verbs.len() as u32;

            // Pagination (convert domain types → infra types)
            let offset = filter.offset as u32;
            let limit = filter.limit as u32;

            // Pagination
            let verbs: Vec<Verb> = verbs
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();

            Ok(VerbListResult { verbs, total })
        })
    }
}
