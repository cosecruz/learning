use std::sync::Arc;

use crate::{
    application::{
        ApplicationError,
        use_cases::{
            CreateVerbUseCase, GetVerbActionLogs, ListVerbsUseCase, TransitionVerbUseCase,
        },
    },
    domain::{
        model::{ActionLog, Verb, VerbId, VerbState},
        repository::{action_log_repo::ActionLogListResult, verb_repo::VerbFilter},
    },
    infra::db::{Database, DatabaseTransaction},
};

/// Facade: Single entry point for all verb operations
///
/// The facade pattern provides a simplified interface over use cases.
/// External code (API, CLI) calls the facade, not use cases directly.
///
/// Benefits:
/// - Decouples clients from internal use case organization
/// - Can add cross-cutting concerns (logging, metrics)
/// - Easier to mock for testing
#[derive(Debug)]
pub struct VerbFacade<D: Database> {
    create_use_case: CreateVerbUseCase<D>,
    transition_use_case: TransitionVerbUseCase<D>,
    list_use_case: ListVerbsUseCase<D>,
    list_verb_logs_use_case: GetVerbActionLogs<D>,
}

impl<D: Database> VerbFacade<D> {
    /// Create a new facade with a database
    pub fn new(db: Arc<D>) -> Self {
        Self {
            create_use_case: CreateVerbUseCase::new(Arc::clone(&db)),
            transition_use_case: TransitionVerbUseCase::new(Arc::clone(&db)),
            list_use_case: ListVerbsUseCase::new(Arc::clone(&db)),
            list_verb_logs_use_case: GetVerbActionLogs::new(Arc::clone(&db)),
        }
    }

    /// Create a new verb
    pub async fn create_verb(
        &self,
        title: String,
        description: String,
    ) -> Result<Verb, ApplicationError> {
        self.create_use_case.execute(title, description).await
    }

    /// Transition verb state
    pub async fn transition_verb(
        &self,
        verb_id: VerbId,
        next_state: VerbState,
        reason: Option<String>,
    ) -> Result<Verb, ApplicationError> {
        self.transition_use_case
            .execute(verb_id, next_state, reason)
            .await
    }

    /// List verbs with filtering
    pub async fn list_verbs(&self, filter: VerbFilter) -> Result<Vec<Verb>, ApplicationError> {
        let result = self.list_use_case.execute(filter).await?;
        Ok(result.verbs)
    }

    /// Get a single verb by ID
    pub async fn get_verb(&self, verb_id: VerbId) -> Result<Verb, ApplicationError> {
        let tx = self
            .create_use_case
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        let verb_repo = tx.verb_repository();

        verb_repo
            .find_by_id(verb_id)
            .await
            .map_err(ApplicationError::from_infra)?
            .ok_or(ApplicationError::NotFound)
    }

    ///Get action logs for a single verb
    pub async fn get_verb_action_logs(
        &self,
        verb_id: VerbId,
        limit: Option<u32>,
    ) -> Result<ActionLogListResult, ApplicationError> {
        let limit = limit.unwrap_or(5);
        let result = self.list_verb_logs_use_case.execute(verb_id, limit).await?;
        Ok(result)
    }

    // Get all logs
}

impl<D: Database> Clone for VerbFacade<D> {
    fn clone(&self) -> Self {
        Self {
            create_use_case: CreateVerbUseCase::new(Arc::clone(&self.create_use_case.db)),
            transition_use_case: TransitionVerbUseCase::new(Arc::clone(
                &self.transition_use_case.db,
            )),
            list_use_case: ListVerbsUseCase::new(Arc::clone(&self.list_use_case.db)),
            list_verb_logs_use_case: GetVerbActionLogs::new(Arc::clone(
                &self.list_verb_logs_use_case.db,
            )),
        }
    }
}
