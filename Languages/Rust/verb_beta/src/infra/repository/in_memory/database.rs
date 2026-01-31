use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::infra::{
    db::{Database, DatabaseError, DatabaseTransaction},
    repository::in_memory::{action_log_repo::InMemoryActionLogRepo, verb_repo::InMemoryVerbRepo},
};

/// In-memory database implementation
///
/// Uses Mutex for thread-safe shared state.
/// Good for testing, not for production.
#[derive(Clone)]
pub struct InMemoryDatabase {
    verb_store: Arc<Mutex<Vec<crate::domain::model::Verb>>>,
    action_log_store: Arc<Mutex<Vec<crate::domain::model::ActionLog>>>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self {
            verb_store: Arc::new(Mutex::new(Vec::new())),
            action_log_store: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl Database for InMemoryDatabase {
    // The GAT: Our transaction type is InMemoryTransaction
    ///
    /// The <'tx> lifetime ties the transaction to the database reference
    type Transaction<'tx>
        = InMemoryTransaction
    where
        Self: 'tx;

    async fn begin_tx(&self) -> Result<Self::Transaction<'_>, DatabaseError> {
        // Clone Arcs - cheap, just incrementing reference counts
        Ok(InMemoryTransaction::new(
            Arc::clone(&self.verb_store),
            Arc::clone(&self.action_log_store),
        ))
    }
}

/// In-memory transaction
///
/// This is a "fake" transaction - it doesn't provide true isolation.
/// Changes are visible immediately. Good enough for testing.
///
/// Note: This struct doesn't hold a lifetime parameter because it owns
/// Arc clones, not references to the database.
pub struct InMemoryTransaction {
    verb_repo: InMemoryVerbRepo,
    action_log_repo: InMemoryActionLogRepo,
}

impl InMemoryTransaction {
    fn new(
        verb_store: Arc<Mutex<Vec<crate::domain::model::Verb>>>,
        action_log_store: Arc<Mutex<Vec<crate::domain::model::ActionLog>>>,
    ) -> Self {
        Self {
            verb_repo: InMemoryVerbRepo::new(verb_store),
            action_log_repo: InMemoryActionLogRepo::new(action_log_store),
        }
    }
}

#[async_trait]
impl DatabaseTransaction for InMemoryTransaction {
    fn action_log_repository(&self) -> &dyn crate::domain::repository::ActionLogRepository {
        &self.action_log_repo
    }

    async fn commit(self) -> Result<(), DatabaseError> {
        // In-memory "commits" immediately, nothing to do
        Ok(())
    }

    fn verb_repository(&self) -> &dyn crate::domain::repository::VerbRepository {
        &self.verb_repo
    }
}
