use std::sync::Arc;

use crate::{
    application::{
        ApplicationError,
        use_cases::{CreateVerbUseCase, ListVerbsUseCase, TransitionVerbUseCase},
    },
    domain::{
        model::{Verb, VerbId, VerbState},
        repository::verb_repo::VerbFilter,
    },
    infra::db::Database,
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
pub struct VerbFacade<D: Database> {
    create_use_case: CreateVerbUseCase<D>,
    transition_use_case: TransitionVerbUseCase<D>,
    list_use_case: ListVerbsUseCase<D>,
}

impl<D: Database> VerbFacade<D> {
    /// Create a new facade with a database
    pub fn new(db: Arc<D>) -> Self {
        Self {
            create_use_case: CreateVerbUseCase::new(Arc::clone(&db)),
            transition_use_case: TransitionVerbUseCase::new(Arc::clone(&db)),
            list_use_case: ListVerbsUseCase::new(Arc::clone(&db)),
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
            .map_err(ApplicationError::from_infra)?
            .ok_or(ApplicationError::NotFound)
    }
}

impl<D: Database> Clone for VerbFacade<D> {
    fn clone(&self) -> Self {
        Self {
            create_use_case: CreateVerbUseCase::new(Arc::clone(&self.create_use_case.db)),
            transition_use_case: TransitionVerbUseCase::new(Arc::clone(
                &self.transition_use_case.db,
            )),
            list_use_case: ListVerbsUseCase::new(Arc::clone(&self.list_use_case.db)),
        }
    }
}
