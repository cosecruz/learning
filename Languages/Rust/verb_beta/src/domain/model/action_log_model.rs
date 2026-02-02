use std::fmt;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{VerbId, VerbState};

/// Strongly-typed ActionLog identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionLogId(Uuid);

impl ActionLogId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ActionLogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Immutable record of a state change.
///
/// Action logs form an append-only audit trail.
/// They are created by the domain when state transitions occur.
#[derive(Debug, Clone)]
pub struct ActionLog {
    id: ActionLogId,
    verb_id: VerbId,
    action_type: ActionType,
    from_state: Option<VerbState>,
    to_state: VerbState,
    reason: Option<String>,
    timestamp: OffsetDateTime,
}

/// Type of action taken on a verb
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Created,
    Activated,
    Paused,
    Completed,
    Dropped,
}

impl ActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::Created => "Created",
            ActionType::Activated => "Activated",
            ActionType::Paused => "Paused",
            ActionType::Completed => "Completed",
            ActionType::Dropped => "Dropped",
        }
    }
}

impl ActionLog {
    /// Create action log for verb creation
    pub fn created(verb_id: VerbId) -> Self {
        Self {
            id: ActionLogId::new(),
            verb_id,
            action_type: ActionType::Created,
            from_state: None,
            to_state: VerbState::Captured,
            reason: None,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    /// Create action log from a state transition
    pub fn from_transition(
        verb_id: VerbId,
        from_state: Option<VerbState>,
        to_state: VerbState,
        reason: Option<String>,
    ) -> Self {
        let action_type = Self::infer_action_type(from_state, to_state);

        Self {
            id: ActionLogId::new(),
            verb_id,
            action_type,
            from_state,
            to_state,
            reason,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    ///private method to help from_transition infer action type from verb previous state and new state
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

    /// Reconstruct from persistence
    pub fn from_parts(
        id: ActionLogId,
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
    // Getters
    pub fn id(&self) -> ActionLogId {
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

//=========================================
#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    fn dummy_verb_id() -> VerbId {
        // adapt if VerbId is not Copy
        VerbId::new()
    }

    fn before_now(ts: OffsetDateTime) -> bool {
        ts <= OffsetDateTime::now_utc()
    }

    // --------------------------------------------------
    // created()
    // --------------------------------------------------

    #[test]
    fn created_action_log_has_correct_defaults() {
        let verb_id = dummy_verb_id();
        let log = ActionLog::created(verb_id);

        assert_eq!(log.verb_id(), verb_id);
        assert_eq!(log.action_type(), ActionType::Created);
        assert_eq!(log.from_state(), None);
        assert_eq!(log.to_state(), VerbState::Captured);
        assert_eq!(log.reason(), None);
        assert!(before_now(log.timestamp()));
    }

    #[test]
    fn created_action_log_has_unique_id() {
        let verb_id = dummy_verb_id();
        let a = ActionLog::created(verb_id);
        let b = ActionLog::created(verb_id);

        assert_ne!(a.id(), b.id());
    }

    // --------------------------------------------------
    // from_transition() – happy paths
    // --------------------------------------------------

    #[test]
    fn transition_captured_to_active_infers_activated() {
        let verb_id = dummy_verb_id();

        let log =
            ActionLog::from_transition(verb_id, Some(VerbState::Captured), VerbState::Active, None);

        assert_eq!(log.action_type(), ActionType::Activated);
        assert_eq!(log.from_state(), Some(VerbState::Captured));
        assert_eq!(log.to_state(), VerbState::Active);
    }

    #[test]
    fn transition_paused_to_active_infers_activated() {
        let verb_id = dummy_verb_id();

        let log = ActionLog::from_transition(
            verb_id,
            Some(VerbState::Paused),
            VerbState::Active,
            Some("waiting".into()),
        );

        assert_eq!(log.action_type(), ActionType::Activated);
    }

    #[test]
    fn transition_active_to_paused_infers_paused() {
        let verb_id = dummy_verb_id();

        let log = ActionLog::from_transition(
            verb_id,
            Some(VerbState::Active),
            VerbState::Paused,
            Some("blocked".into()),
        );

        assert_eq!(log.action_type(), ActionType::Paused);
        assert_eq!(log.reason(), Some("blocked"));
    }

    #[test]
    fn transition_active_to_done_infers_completed() {
        let verb_id = dummy_verb_id();

        let log =
            ActionLog::from_transition(verb_id, Some(VerbState::Active), VerbState::Done, None);

        assert_eq!(log.action_type(), ActionType::Completed);
    }

    #[test]
    fn transition_any_to_dropped_infers_dropped() {
        let verb_id = dummy_verb_id();

        let log = ActionLog::from_transition(
            verb_id,
            Some(VerbState::Active),
            VerbState::Dropped,
            Some("no longer relevant".into()),
        );

        assert_eq!(log.action_type(), ActionType::Dropped);
    }

    // --------------------------------------------------
    // Edge cases & fallback behavior
    // --------------------------------------------------

    #[test]
    fn none_to_captured_infers_created() {
        let verb_id = dummy_verb_id();

        let log = ActionLog::from_transition(verb_id, None, VerbState::Captured, None);

        assert_eq!(log.action_type(), ActionType::Created);
    }

    #[test]
    fn unknown_transition_falls_back_to_activated() {
        let verb_id = dummy_verb_id();

        let log =
            ActionLog::from_transition(verb_id, Some(VerbState::Done), VerbState::Paused, None);

        assert_eq!(log.action_type(), ActionType::Activated);
    }

    // --------------------------------------------------
    // from_parts()
    // --------------------------------------------------

    #[test]
    fn from_parts_reconstructs_exactly() {
        let id = ActionLogId::new();
        let verb_id = dummy_verb_id();
        let timestamp = OffsetDateTime::now_utc();

        let log = ActionLog::from_parts(
            id,
            verb_id,
            ActionType::Completed,
            Some(VerbState::Active),
            VerbState::Done,
            None,
            timestamp,
        );

        assert_eq!(log.id(), id);
        assert_eq!(log.verb_id(), verb_id);
        assert_eq!(log.action_type(), ActionType::Completed);
        assert_eq!(log.from_state(), Some(VerbState::Active));
        assert_eq!(log.to_state(), VerbState::Done);
        assert_eq!(log.reason(), None);
        assert_eq!(log.timestamp(), timestamp);
    }

    // --------------------------------------------------
    // Getter invariants
    // --------------------------------------------------

    #[test]
    fn getters_do_not_mutate_state() {
        let verb_id = dummy_verb_id();
        let log = ActionLog::created(verb_id);

        let _ = log.id();
        let _ = log.verb_id();
        let _ = log.action_type();
        let _ = log.from_state();
        let _ = log.to_state();
        let _ = log.reason();
        let _ = log.timestamp();

        // if this compiles and does not panic, the test passes
        assert_eq!(log.action_type(), ActionType::Created);
    }
}
