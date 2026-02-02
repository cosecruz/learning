use std::pin::Pin;

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, VerbId},
};

// ==================================================
// ACTION_LOG REPOSITORY TRAIT
// ==================================================
pub trait ActionLogRepository: Send + Sync {
    /// Append an action log entry
    fn append(
        &self,
        log: &ActionLog,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    /// Get action logs for a specific verb
    fn find_by_verb(
        &self,
        verb_id: VerbId,
        limit: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ActionLog>, ApplicationError>> + Send + '_>>;
}

#[derive(Debug, Clone)]
pub struct ActionLogListResult {
    pub action_logs: Vec<ActionLog>,
    pub total: u32,
}

// #[derive(Debug, Clone)]
// pub struct ActionLogFilter {
//     pub verb_id: ,
//     pub limit: u32,
//     pub offset: u32,
// }
