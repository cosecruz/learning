use std::sync::Arc;

use crate::{
    application::error::ApplicationError,
    domain::repository::verb_repo::{VerbFilter, VerbListResult},
    infra::db::Database,
};

/// Use case: List verbs with filtering
///
/// This is a read-only operation, so no transaction needed.
pub struct ListVerbsUseCase<D: Database> {
    pub(crate) db: Arc<D>,
}

impl<D: Database> ListVerbsUseCase<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, filter: VerbFilter) -> Result<VerbListResult, ApplicationError> {
        // Read-only operations don't need transactions
        let tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        let verb_repo = tx.verb_repository();

        let result = verb_repo
            .list(filter)
            .map_err(ApplicationError::from_infra)?;

        // No commit needed for read-only
        Ok(result)
    }
}
