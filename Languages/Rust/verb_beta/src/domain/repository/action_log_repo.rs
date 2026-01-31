use std::{error::Error, pin::Pin};

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, VerbId},
};

// ==================================================
// ACTION_LOG REPOSITORY TRAIT
// ==================================================
pub trait ActionLogRepository: Send + Sync + 'static {
    /// Append an action log entry
    fn append(
        &self,
        log: &ActionLog,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    /// Get action logs for a specific verb
    fn find_by_verb(
        &self,
        verb_id: VerbId,
        limit: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ActionLog>, ApplicationError>> + Send + '_>>;
}
