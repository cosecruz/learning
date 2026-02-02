use std::sync::Arc;

use crate::{
    application::ApplicationError,
    domain::{
        model::VerbId,
        repository::action_log_repo::{ActionLogFilter, ActionLogListResult},
    },
    infra::db::{Database, DatabaseTransaction},
};

#[derive(Debug, Clone)]
pub struct GetVerbActionLogs<D: Database> {
    pub db: Arc<D>,
}

impl<D: Database> GetVerbActionLogs<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        verb_id: VerbId,
        filter: &ActionLogFilter,
    ) -> Result<ActionLogListResult, ApplicationError> {
        // Begin transaction (even for reads - ensures consistent snapshot)
        let tx = self
            .db
            .begin_tx()
            .await
            .map_err(|e| ApplicationError::Transaction(e.to_string()))?;

        let action_log_repo = tx.action_log_repository();

        // Execute Query (async)
        let result = action_log_repo.find_by_verb(verb_id, filter).await?;

        // No explicit commit needed for read-only
        // Transaction will be dropped (rolled back implicitly)

        let length = result.len() as u32;
        let res = ActionLogListResult {
            action_logs: result,
            total: length,
        };
        Ok(res)
    }
}
