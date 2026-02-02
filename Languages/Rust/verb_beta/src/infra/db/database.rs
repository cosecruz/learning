use std::pin::Pin;

use async_trait::async_trait;

use crate::{
    domain::repository::{ActionLogRepository, VerbRepository},
    infra::db::DatabaseError,
};

/// Database trait using GAT for proper type-safe transactions
///
/// ## Why GAT?
/// Generic Associated Types allow each database implementation to define
/// its own transaction type while maintaining lifetime correctness.
///
/// ## Why not `async fn`?
/// `async fn begin_tx(&self)` would make the trait NOT object-safe.
/// We use `Pin<Box<dyn Future>>` to maintain object safety while supporting async.
///
/// ## Object Safety
/// This trait IS object-safe because:
/// - No `Self: Sized` methods
/// - No generic methods
/// - Associated type has `where Self: 'tx` bounds
pub trait Database: Clone + Send + Sync + 'static {
    /// The transaction type for this database
    ///
    /// GAT allows each implementation to specify its concrete transaction type.
    /// The `'tx` lifetime ties the transaction to the database reference.
    type Transaction<'tx>: DatabaseTransaction
    where
        Self: 'tx;

    /// Begin a new transaction
    ///
    /// Returns a boxed future for object safety.
    /// The `'_` lifetime in `Self::Transaction<'_>` is inferred from `&self`.
    fn begin_tx(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Transaction<'_>, DatabaseError>> + Send + '_>>;
}

// ===========================================================
/// Transaction trait for scoped unit of work
///
/// ## Why NOT object-safe?
/// This trait is NOT object-safe because:
/// - `commit(self)` consumes self by value
/// - Cannot be called through `&dyn DatabaseTransaction`
///
/// This is OK because transactions are used with concrete types via GAT,
/// not through trait objects.
pub trait DatabaseTransaction: Send {
    /// Get verb repository for this transaction
    fn verb_repository(&self) -> &dyn VerbRepository;

    /// Get action log repository for this transaction
    fn action_log_repository(&self) -> &dyn ActionLogRepository;

    /// Commit this transaction
    ///
    /// Takes `self` by value to consume the transaction.
    /// Returns boxed future for consistency.
    fn commit(self) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + 'static>>;

    /// Rollback this transaction
    ///
    /// Default implementation does nothing (relies on Drop).
    fn rollback(self) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + 'static>>
    where
        Self: Sized,
    {
        Box::pin(async { Ok(()) })
    }
}
