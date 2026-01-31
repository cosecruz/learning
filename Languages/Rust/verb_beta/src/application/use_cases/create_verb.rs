use std::sync::Arc;

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, Verb},
    infra::db::Database,
};

// Use case: Create a new verb
///
/// Responsibilities:
/// 1. Create verb entity (domain validates)
/// 2. Create initial action log
/// 3. Open transaction
/// 4. Save both atomically
/// 5. Commit transaction
pub struct CreateVerbUseCase<D: Database> {
    pub(crate) db: Arc<D>,
}

impl<D: Database> CreateVerbUseCase<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    /// Execute the use case
    ///
    /// Transaction boundary is HERE, not in the repository.
    /// This ensures both verb and action log are saved atomically.
    pub async fn execute(
        &self,
        title: String,
        description: String,
    ) -> Result<Verb, ApplicationError> {
        // Step 1: Create domain entities (validates)
        let verb = Verb::new(title, description)?;
        let action_log = ActionLog::created(verb.id());

        // Step 2: Begin transaction
        let mut tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        // Step 3: Get repositories (bound to this transaction)
        let verb_repo = tx.verb_repository();
        let log_repo = tx.action_log_repository();

        // Step 4: Save both within transaction
        verb_repo
            .save(&verb)
            .map_err(ApplicationError::from_infra)?;

        log_repo
            .append(&action_log)
            .map_err(ApplicationError::from_infra)?;

        // Step 5: Commit transaction
        tx.commit()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        Ok(verb)
    }
}
