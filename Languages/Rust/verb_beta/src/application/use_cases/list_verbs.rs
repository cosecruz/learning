use std::sync::Arc;

use crate::{
    application::error::ApplicationError,
    domain::repository::verb_repo::{VerbFilter, VerbListResult},
    infra::db::{Database, DatabaseTransaction},
};

/// Use case: List verbs with filtering
///
/// This is a read-only operation, so no transaction needed.
#[derive(Debug, Clone)]
pub struct ListVerbsUseCase<D: Database> {
    pub db: Arc<D>,
}

impl<D: Database> ListVerbsUseCase<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, filter: VerbFilter) -> Result<VerbListResult, ApplicationError> {
        // Begin transaction (even for reads - ensures consistent snapshot)
        let tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        let verb_repo = tx.verb_repository();

        // Execute query (async)
        let result = verb_repo
            .list(filter)
            .await
            .map_err(ApplicationError::from_infra)?;

        // No explicit commit needed for read-only
        // Transaction will be dropped (rolled back implicitly)

        Ok(result)
    }
}
