//! Verb Models
use std::fmt;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{error::DomainError, model::ActionLog};

// ============================================================================
// Value Objects
// ============================================================================
/// Strongly-typed identity for a Verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VerbId(Uuid);

impl VerbId {
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

impl fmt::Display for VerbId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for VerbId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

// ============================================================================
#[derive(Debug, Clone)]
struct Title(String);

impl Title {
    fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let v = value.into().trim().to_string();
        if v.is_empty() {
            return Err(DomainError::VerbEmptyTitle);
        }
        if v.len() > 200 {
            return Err(DomainError::VerbTitleTooLong);
        }
        Ok(Self(v))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================

#[derive(Debug, Clone)]
struct Description(String);

impl Description {
    fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let v = value.into();
        if v.len() > 2000 {
            return Err(DomainError::VerbDescriptionTooLong);
        }
        Ok(Self(v))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// Verb Entity
// ============================================================================
/// Domain entity representing a user's intent over time.
///
/// This entity encapsulates all business rules around:
/// - Creation validation
/// - State transitions
/// - Temporal consistency
#[derive(Debug, Clone)]
pub struct Verb {
    id: VerbId,
    title: Title,
    description: Description,
    state: VerbState,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Verb {
    /// Create a new verb in Captured state.
    ///
    /// This is a **factory method** that enforces invariants:
    /// - Title must be 1-200 chars
    /// - Description must be 0-2000 chars
    /// - Initial state is always Captured
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let now = OffsetDateTime::now_utc();
        Ok(Self {
            id: VerbId::new(),
            title: Title::new(title)?,
            description: Description::new(description)?,
            state: VerbState::Captured,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstruct a verb from persistence (bypasses validation).
    ///
    /// Used by repositories when loading from database.
    /// Assumes data was validated when originally created.
    pub fn from_parts(
        id: VerbId,
        title: String,
        description: String,
        state: VerbState,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            title: Title::new(title)?,
            description: Description::new(description)?,
            state,
            created_at,
            updated_at,
        })
    }

    // ========================================================================
    // Getters (domain entities expose their state)
    // ========================================================================
    pub fn id(&self) -> VerbId {
        self.id
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn state(&self) -> VerbState {
        self.state
    }
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}

// ============================================================================
// State Machine
// ============================================================================

///VerbState : The lifecycle stage of a verb.
/// States:
///
///- `Captured` — intent recorded, not yet acted on
///- `Active` — user has started
///- `Paused` — temporarily stopped, with reason
///- `Done` — completed
///- `Dropped` — explicitly abandoned, with reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerbState {
    Captured,
    Active,
    Paused,
    Done,
    Dropped,
}

impl VerbState {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerbState::Captured => "Captured",
            VerbState::Active => "Active",
            VerbState::Paused => "Paused",
            VerbState::Done => "Done",
            VerbState::Dropped => "Dropped",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Captured" => Ok(VerbState::Captured),
            "Active" => Ok(VerbState::Active),
            "Paused" => Ok(VerbState::Paused),
            "Done" => Ok(VerbState::Done),
            "Dropped" => Ok(VerbState::Dropped),
            _ => Err(DomainError::VerbInvalidState(s.to_string())),
        }
    }
}

impl fmt::Display for VerbState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// State Transitions
// ============================================================================

impl Verb {
    //========================================================================
    // State Machine (core business logic)
    // ========================================================================
    /// **Valid Transitions**
    ///- Captured → Active
    ///- Paused → Active
    ///
    ///- Active → Paused
    ///- Active → Done
    ///
    ///- Done → Active (re-open)
    ///- Dropped → Active (re-open)
    ///
    ///- Captured → Dropped
    ///- Active → Dropped
    ///- Paused → Dropped
    ///
    /// Transition abstraction to check if state transition is valid
    ///
    /// Check if transition to next state is valid.
    ///
    /// State machine rules:
    /// - Captured -> Active, Dropped
    /// - Active -> Paused, Done, Dropped
    /// - Paused -> Active, Dropped
    ///
    /// Invalid transitions return false (domain protects its invariants).
    pub fn can_transition_to(&self, next: VerbState) -> bool {
        use VerbState::*;
        matches!(
            (self.state, next),
            (Captured, Active)
                | (Captured, Dropped)
                | (Active, Paused)
                | (Active, Done)
                | (Active, Dropped)
                | (Paused, Active)
                | (Paused, Dropped)
                | (Done, Active)
                | (Dropped, Active)
        )
    }

    /// Transition to next state, producing an ActionLog.
    ///
    /// This method:
    /// 1. Validates the transition
    /// 2. Updates internal state
    /// 3. Returns an ActionLog (event)
    ///
    /// The ActionLog MUST be persisted atomically with the verb.
    /// This is enforced by the application layer via transactions.
    pub fn transition_to(
        &mut self,
        next: VerbState,
        reason: Option<String>,
    ) -> Result<ActionLog, DomainError> {
        // Validate reason length
        if let Some(ref r) = reason
            && r.len() > 500
        {
            return Err(DomainError::ReasonTooLong);
        }

        // Check transition validity
        if !self.can_transition_to(next) {
            return Err(DomainError::InvalidTransition {
                from: self.state,
                to: next,
            });
        }

        let from_state = self.state;
        self.state = next;
        self.updated_at = OffsetDateTime::now_utc();

        Ok(ActionLog::from_transition(
            self.id,
            Some(from_state),
            next,
            reason,
        ))
    }
}

//================================================
// UNIT TESTS
//================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_verb_starts_in_captured_state() {
        let verb = Verb::new("Test", "Description").unwrap();
        assert_eq!(verb.state(), VerbState::Captured);
    }

    #[test]
    fn rejects_empty_title() {
        let result = Verb::new("", "Desc");
        assert!(matches!(result, Err(DomainError::VerbEmptyTitle)));
    }

    #[test]
    fn trims_title() {
        let verb = Verb::new("  Test  ", "Desc").unwrap();
        assert_eq!(verb.title(), "Test");
    }

    #[test]
    fn can_activate_from_captured() {
        let verb = Verb::new("Test", "Desc").unwrap();
        assert!(verb.can_transition_to(VerbState::Active));
    }

    #[test]
    fn cannot_complete_from_captured() {
        let verb = Verb::new("Test", "Desc").unwrap();
        assert!(!verb.can_transition_to(VerbState::Done));
    }

    #[test]
    fn transition_updates_state_and_timestamp() {
        let mut verb = Verb::new("Test", "Desc").unwrap();
        let old_updated_at = verb.updated_at();

        std::thread::sleep(std::time::Duration::from_millis(10));

        verb.transition_to(VerbState::Active, None).unwrap();

        assert_eq!(verb.state(), VerbState::Active);
        assert!(verb.updated_at() > old_updated_at);
    }
}

//CRUD Use-Case
//  - Create a verb (description only)
//  - List all verbs
//  - View a single verb
//  - Update verb state (`Captured → Active → Done`)
//  - Drop a verb (with optional reason)
//  - View the action log for a verb
//
// Persistence
// - Write verbs and actions to disk
// - Query by state, date, tags
// - Ensure atomic writes

//use cases needed to implement are
// - creating a verb
// user: create new verb: payload(title, description)
// create verb model with new -> action should match -> save: model data and action to disk on verb_store and action_logs

// - list all verbs
// user: get all verbs
// query from store all verbs
// paginate result where? directly from store or at another layer
// return result

// - view a single verb
// user: get verb

// query verb :
// - by id
// - by state: default: all, by selected
// - by date: asc, desc, specific, range eg: by week, month, year, time of ? when to when, updated_at, created_at, state_changed_at
// - by action: lru, lfu, last_change, changed_to-what?
//  - by tags: future
// -by search: grep store for matches, similar to desc
// - by title, desc, context
// - by combination of query

// update verb state:
//query by id -> update state using verb model which validates state transitions-> if valid update -> return updated verb
// can also do like live response kind of where system will live feed user valid state changes they can do based on their current state

// update verb title, desc, context, tags
// query by id -> validate updated verb model -> update if valid -> return updated verb
//  can also live feedback to interfaces

// drop a verb
// if user wants to delete verb
// system does soft delete
// verb is changed to dropped and put in DroppedVerbs with time it was put their
// after specific time background job cleans it; then its gone
// or use can delete and enter then name of the verb to hard delete

// view action log
// 30days worth of logs
// there should action_log layers and background to downgrade layer to lower layers
// raw: as is view as it is
// queried by id, title, desc, context and others: lru
