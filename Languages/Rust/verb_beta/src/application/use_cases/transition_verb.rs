use std::sync::Arc;

use crate::{
    application::error::ApplicationError,
    domain::{
        model::{Verb, VerbId, VerbState},
        repository::{ActionLogRepository, VerbRepository},
    },
    infra::db::{Database, DatabaseTransaction},
};

/// Use case: Transition a verb to a new state
///
/// Responsibilities:
/// 1. Load verb
/// 2. Validate transition (domain)
/// 3. Execute transition (produces action log)
/// 4. Save both atomically
pub struct TransitionVerbUseCase<D: Database> {
    pub(crate) db: Arc<D>,
}

impl<D: Database> TransitionVerbUseCase<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        verb_id: VerbId,
        next_state: VerbState,
        reason: Option<String>,
    ) -> Result<Verb, ApplicationError> {
        // Begin transaction
        let mut tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        // Get repositories
        let verb_repo = tx.verb_repository();
        let log_repo = tx.action_log_repository();

        // Load verb
        let mut verb = verb_repo
            .find_by_id(verb_id)
            .map_err(ApplicationError::from_infra)?
            .ok_or(ApplicationError::NotFound)?;

        // Execute transition (domain validates)
        let action_log = verb.transition_to(next_state, reason)?;

        // Save both
        verb_repo
            .save(&verb)
            .map_err(ApplicationError::from_infra)?;

        log_repo
            .append(&action_log)
            .map_err(ApplicationError::from_infra)?;

        // Commit
        tx.commit()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        Ok(verb)
    }
}
