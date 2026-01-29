use time::OffsetDateTime;
use uuid::Uuid;

use crate::services::core::model::verb::{Reason, VerbId, VerbState};

///What happened and when it happened
/// A discrete, immutable fact describing something that happened in the system.
#[derive(Debug, Clone)]
pub struct ActionLog {
    id: Uuid,
    verb_id: VerbId,
    action_type: ActionType,
    from_state: Option<VerbState>, // None for Created
    to_state: VerbState,
    reason: Option<String>, // Max 500 chars
    timestamp: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Created,
    Activated,
    Paused,
    Completed,
    Dropped,
}

impl ActionLog {
    /// new associated method
    pub fn new(
        verb_id: VerbId,
        action_type: ActionType,
        from_state: Option<VerbState>,
        to_state: VerbState,
        reason: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            verb_id,
            action_type,
            from_state,
            to_state,
            reason,
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }
}
