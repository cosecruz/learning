use std::pin::Pin;

use serde::Serialize;

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, ActionType, VerbId, VerbState},
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
    //TODO: Use a named lifetime not static
    fn find_by_verb(
        &self,
        verb_id: VerbId,
        filter: &ActionLogFilter,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ActionLog>, ApplicationError>> + Send + '_>>;
}

#[derive(Debug, Clone)]
pub struct ActionLogFilter {
    pub state: Option<ActionType>,
    pub limit: u32,
    pub offset: u32,
}

impl Default for ActionLogFilter {
    fn default() -> Self {
        Self {
            state: None,
            limit: 50,
            offset: 0,
        }
    }
}

impl ActionLogFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(mut self, state: ActionType) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit.min(200); // Cap at 200
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
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
