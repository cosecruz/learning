use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{VerbId, VerbState};

///What happened and when it happened
/// A discrete, immutable fact describing something that happened in the system.
#[derive(Debug, Clone)]
pub struct ActionLog {
    id: Uuid,
    verb_id: VerbId,
    action_type: ActionType,
    from_state: Option<VerbState>,
    to_state: VerbState,
    reason: Option<String>,
    timestamp: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Created,
    Activated,
    Paused,
    Completed,
    Dropped,
}

impl ActionLog {
    pub fn created(verb_id: VerbId) -> Self {
        Self {
            id: Uuid::new_v4(),
            verb_id,
            action_type: ActionType::Created,
            from_state: None,
            to_state: VerbState::Captured,
            reason: None,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    pub fn from_transition(
        verb_id: VerbId,
        from_state: Option<VerbState>,
        to_state: VerbState,
        reason: Option<String>,
    ) -> Self {
        let action_type = Self::infer_action_type(from_state, to_state);

        Self {
            id: Uuid::new_v4(),
            verb_id,
            action_type,
            from_state,
            to_state,
            reason,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    fn infer_action_type(from: Option<VerbState>, to: VerbState) -> ActionType {
        use VerbState::*;
        match (from, to) {
            (None, Captured) => ActionType::Created,
            (Some(Captured | Paused), Active) => ActionType::Activated,
            (Some(Active), Paused) => ActionType::Paused,
            (Some(Active), Done) => ActionType::Completed,
            (_, Dropped) => ActionType::Dropped,
            _ => ActionType::Activated,
        }
    }

    // Reconstruct from persistence
    pub fn from_parts(
        id: Uuid,
        verb_id: VerbId,
        action_type: ActionType,
        from_state: Option<VerbState>,
        to_state: VerbState,
        reason: Option<String>,
        timestamp: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            verb_id,
            action_type,
            from_state,
            to_state,
            reason,
            timestamp,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn verb_id(&self) -> VerbId {
        self.verb_id
    }
    pub fn action_type(&self) -> ActionType {
        self.action_type
    }
    pub fn from_state(&self) -> Option<VerbState> {
        self.from_state
    }
    pub fn to_state(&self) -> VerbState {
        self.to_state
    }
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    pub fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }
}
