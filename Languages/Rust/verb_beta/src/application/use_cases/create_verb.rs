use std::sync::Arc;

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, Verb},
    infra::db::{Database, DatabaseTransaction},
};

// Use case: Create a new verb
///
/// Responsibilities:
/// 1. Create verb entity (domain validates)
/// 2. Create initial action log
/// 3. Open transaction
/// 4. Save both atomically
/// 5. Commit transaction
#[derive(Debug, Clone)]
pub struct CreateVerbUseCase<D: Database> {
    pub db: Arc<D>,
}

impl<D: Database> CreateVerbUseCase<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    /// Execute the use case
    ///
    /// ## Why async?
    /// - Calls async repository methods
    /// - Manages async transaction lifecycle
    pub async fn execute(
        &self,
        title: String,
        description: String,
    ) -> Result<Verb, ApplicationError> {
        // Step 1: Create domain entities (synchronous - domain validates)
        let verb = Verb::new(title, description)?;
        let action_log = ActionLog::created(verb.id());

        // Step 2: Begin transaction (async)
        let tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        // Step 3: Get repositories from transaction
        let verb_repo = tx.verb_repository();
        let log_repo = tx.action_log_repository();

        // Step 4: Save both within transaction (async)
        verb_repo
            .save(&verb)
            .await
            .map_err(ApplicationError::from_infra)?;

        log_repo
            .append(&action_log)
            .await
            .map_err(ApplicationError::from_infra)?;

        // Step 5: Commit transaction (async)
        tx.commit()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        Ok(verb)
    }
}
