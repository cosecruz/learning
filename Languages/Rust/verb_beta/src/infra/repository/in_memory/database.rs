use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::infra::{
    db::{Database, DatabaseError, DatabaseTransaction},
    repository::in_memory::{action_log_repo::InMemoryActionLogRepo, verb_repo::InMemoryVerbRepo},
};

/// In-memory database implementation
///
/// Uses `Arc<Mutex<Vec<T>>>` for thread-safe shared state.
/// Good for testing and single-user scenarios.
///
/// ## Design Notes:
/// - Cloning is cheap (only increments Arc reference count)
/// - Mutex provides interior mutability
/// - Not suitable for high-concurrency production use
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

impl Database for InMemoryDatabase {
    /// GAT: Our transaction type is InMemoryTransaction
    ///
    /// The `'tx` lifetime parameter allows the transaction to borrow from self.
    /// However, InMemoryTransaction doesn't actually need 'tx because it owns Arc clones.
    type Transaction<'tx>
        = InMemoryTransaction
    where
        Self: 'tx;

    fn begin_tx(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Transaction<'_>, DatabaseError>> + Send + '_>>
    {
        // Clone Arcs (cheap - just reference count increment)
        let verb_store = Arc::clone(&self.verb_store);
        let action_log_store = Arc::clone(&self.action_log_store);

        // Return boxed future for object safety
        Box::pin(async move { Ok(InMemoryTransaction::new(verb_store, action_log_store)) })
    }
}

/// In-memory transaction
///
/// This is a "fake" transaction - it doesn't provide true ACID isolation.
/// Changes are visible immediately to other transactions.
///
/// ## Why no lifetime parameter?
/// Even though Database::Transaction<'tx> has a lifetime parameter,
/// InMemoryTransaction doesn't use it because it owns Arc clones,
/// not borrows from the database.
pub struct InMemoryTransaction {
    verb_repo: InMemoryVerbRepo,
    action_log_repo: InMemoryActionLogRepo,
}

impl InMemoryTransaction {
    pub fn new(
        verb_store: Arc<Mutex<Vec<crate::domain::model::Verb>>>,
        action_log_store: Arc<Mutex<Vec<crate::domain::model::ActionLog>>>,
    ) -> Self {
        Self {
            verb_repo: InMemoryVerbRepo::new(verb_store),
            action_log_repo: InMemoryActionLogRepo::new(action_log_store),
        }
    }
}

impl DatabaseTransaction for InMemoryTransaction {
    fn verb_repository(&self) -> &dyn crate::domain::repository::VerbRepository {
        &self.verb_repo
    }

    fn action_log_repository(&self) -> &dyn crate::domain::repository::ActionLogRepository {
        &self.action_log_repo
    }

    fn commit(self) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + 'static>> {
        // In-memory "commits" immediately, nothing to do
        Box::pin(async { Ok(()) })
    }
}
