use std::pin::Pin;

use async_trait::async_trait;

use crate::{
    domain::repository::{ActionLogRepository, VerbRepository},
    infra::db::DatabaseError,
};

// Database trait: Defines what a database must provide
///
/// Core database port (generic, strongly typed)
/// This is the main **port** for persistence infrastructure.
/// Different implementations (SQLite, in-memory) implement this trait.
#[async_trait]
pub trait Database: Send + Sync + 'static {
    // The transaction type for this database
    ///
    /// GAT syntax: type Transaction<'a> means "a type that can have a lifetime 'a"
    /// The `where Self: 'a` ensures the transaction can't outlive the database
    type Transaction<'tx>: DatabaseTransaction + 'tx
    where
        Self: 'tx;
    /// Begin a new transaction
    ///
    /// Returns Self::Transaction<'_>, meaning a transaction tied to &self's lifetime
    async fn begin_tx(&self) -> Result<Self::Transaction<'_>, DatabaseError>;
}

/// Transaction trait: Scoped unit of work
///
/// Transactions provide:
/// 1. Repositories bound to this transaction
/// 2. Commit/rollback semantics
/// 3. Isolation from other transactions
#[async_trait]
pub trait DatabaseTransaction: Send + 'static {
    /// Get verb repository for this transaction
    fn verb_repository(&self) -> &dyn VerbRepository;

    /// Get action log repository for this transaction
    fn action_log_repository(&self) -> &dyn ActionLogRepository;

    /// Commit this transaction
    ///
    /// Takes self by value to consume the transaction
    async fn commit(self) -> Result<(), DatabaseError>;

    /// Rollback this transaction (implicit via Drop if not called)
    async fn rollback(&self) -> Result<(), DatabaseError> {
        // Default: do nothing, relies on Drop
        Ok(())
    }
    // fn commit(&self) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send + '_>>;
    //  fn commit<T: ApplicationError>(&self) -> Pin<Box<dyn Future<Output = Result<(), T>> + Send>>;
}
