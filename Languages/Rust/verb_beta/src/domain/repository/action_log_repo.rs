use crate::domain::{
    error::DomainError,
    model::{ActionLog, VerbId},
};

// ==================================================
// ACTION_LOG REPOSITORY TRAIT
// ==================================================
pub trait ActionLogRepository: Send + Sync {
    /// Get action logs for a specific verb
    async fn get_for_verb(
        &self,
        verb_id: VerbId,
        limit: u32,
    ) -> Result<Vec<ActionLog>, DomainError>;
}
