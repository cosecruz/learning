use std::fmt;

use crate::services::core::error::CoreError;
use time::OffsetDateTime;

/// Strongly-typed identity for a Verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerbId(pub u64);

///Strongly-typed description for a Verb.
#[derive(Debug, Clone)]
pub struct Description(String);

impl Description {
    ///use new to validate description on creation
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let v = value.into();
        if v.trim().is_empty() {
            Err(CoreError::EmptyDescription)
        } else {
            Ok(Self(v))
        }
    }
}

/// Domain entity representing a user's intent over time.
#[derive(Debug, Clone)]
pub struct Verb {
    id: VerbId,
    title: String,
    description: Description,
    state: VerbState,

    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,

    context: Option<String>,
}

impl Verb {
    ///Create a new verb in the Captured state
    pub fn new(
        id: VerbId,
        title: impl Into<String>,
        description: impl Into<String>,
        now: OffsetDateTime,
    ) -> Result<Self, CoreError> {
        let description = Description::new(description.into())?;
        Ok(Self {
            id,
            title: title.into(),
            description,
            state: VerbState::Captured,
            created_at: now,
            updated_at: now,
            context: None,
        })
    }

    pub fn id(&self) -> VerbId {
        self.id
    }

    pub fn state(&self) -> &VerbState {
        &self.state
    }

    // pub fn activate(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
    //     let curr_state = self.state.clone();
    //     let next_state = VerbState::Active;
    //     match self.state {
    //         VerbState::Captured | VerbState::Paused(_) => self.transition_to(next_state, now),
    //         _ => Err(CoreError::InvalidStateTransition {
    //             from: curr_state,
    //             to: next_state,
    //         }),
    //     }
    // }

    // pub fn pause(
    //     &mut self,
    //     reason: Reason,
    //     now: OffsetDateTime,
    // ) -> Result<StateChanged, CoreError> {
    //     let curr_state = self.state.clone();
    //     let next_state = VerbState::Paused(reason);
    //     match self.state {
    //         VerbState::Active => self.transition_to(next_state, now),
    //         _ => Err(CoreError::InvalidStateTransition {
    //             from: curr_state,
    //             to: next_state,
    //         }),
    //     }
    // }

    // pub fn complete(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
    //     let curr_state = self.state.clone();
    //     let next_state = VerbState::Done;
    //     match self.state {
    //         VerbState::Active => self.transition_to(next_state, now),
    //         _ => Err(CoreError::InvalidStateTransition {
    //             from: curr_state,
    //             to: next_state,
    //         }),
    //     }
    // }

    // pub fn drop(
    //     &mut self,
    //     reason: Reason,
    //     now: OffsetDateTime,
    // ) -> Result<StateChanged, CoreError> {
    //     let curr_state = self.state.clone();
    //     let next_state = VerbState::Dropped(reason);
    //     match self.state {
    //         VerbState::Done => Err(CoreError::InvalidStateTransition {
    //             from: curr_state,
    //             to: next_state,
    //         }),
    //         _ => self.transition_to(next_state, now),
    //     }
    // }

    // pub fn reopen(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
    //     let curr_state = self.state.clone();
    //     let next_state = VerbState::Active;
    //     match self.state {
    //         VerbState::Done => self.transition_to(next_state, now),
    //         _ => Err(CoreError::InvalidStateTransition {
    //             from: curr_state,
    //             to: next_state,
    //         }),
    //     }
    // }

    fn transition_to(
        &mut self,
        next: VerbState,
        now: OffsetDateTime,
    ) -> Result<StateChanged, CoreError> {
        if now < self.updated_at {
            return Err(CoreError::InvalidTimeStamp);
        }
        let from = self.state.clone();
        self.state = next.clone();
        self.updated_at = now;

        Ok(StateChanged::new(self.id, from, next, now))
    }
}

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
impl Verb {
    fn ensure_can_transition_to(&self, next: &VerbState) -> Result<(), CoreError> {
        use VerbStateKind::*;

        let from = self.state.clone();
        let to = next.clone();

        let from_kind = from.kind();
        let to_kind = to.kind();

        let allowed = match (from_kind, to_kind) {
            // Captured | Paused → Active
            (Captured | Paused, Active) => true,

            // Active → Paused|Done
            (Active, Paused | Done) => true,

            // Done|Dropped → Active (reopen)
            (Done | Dropped, Active) => true,

            // Any except Done → Dropped
            (_, Dropped) if from_kind != Done => true,

            _ => false,
        };

        if allowed {
            Ok(())
        } else {
            Err(CoreError::InvalidStateTransition { from, to })
        }
    }
}

/// Transition Behaviors
impl Verb {
    /// Transition: Captured | Paused → Active
    pub fn activate(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
        self.apply_transition(VerbState::Active, now)
    }

    /// Transition: Active → Paused(reason)
    pub fn pause(
        &mut self,
        reason: Reason,
        now: OffsetDateTime,
    ) -> Result<StateChanged, CoreError> {
        self.apply_transition(VerbState::Paused(reason), now)
    }

    /// Transition: Active → Done
    pub fn complete(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
        self.apply_transition(VerbState::Done, now)
    }

    /// Transition: Any (except Done) → Dropped(reason)
    pub fn drop(&mut self, reason: Reason, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
        self.apply_transition(VerbState::Dropped(reason), now)
    }

    /// Re-open a completed verb.
    /// Transition: Done|Dropped → Active
    pub fn reopen(&mut self, now: OffsetDateTime) -> Result<StateChanged, CoreError> {
        self.apply_transition(VerbState::Active, now)
    }
}

///apply transition to state if it is valid
impl Verb {
    fn apply_transition(
        &mut self,
        next: VerbState,
        now: OffsetDateTime,
    ) -> Result<StateChanged, CoreError> {
        //Idempotency guaranteed
        if self.state.kind() == next.kind() {
            return Ok(StateChanged::new(self.id, self.state.clone(), next, now));
        }
        self.ensure_can_transition_to(&next)?;
        self.transition_to(next, now)
    }
}

///VerbState : The lifecycle stage of a verb.
/// States:
///
///- `Captured` — intent recorded, not yet acted on
///- `Active` — user has started
///- `Paused` — temporarily stopped, with reason
///- `Done` — completed
///- `Dropped` — explicitly abandoned, with reason
///
#[derive(Debug, Clone)]
pub enum VerbState {
    Captured,
    Active,
    Paused(Reason),
    Done,
    Dropped(Reason),
}

impl fmt::Display for VerbState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerbState::Captured => write!(f, "captured"),
            VerbState::Active => write!(f, "active"),
            VerbState::Paused(_) => write!(f, "paused"),
            VerbState::Done => write!(f, "done"),
            VerbState::Dropped(_) => write!(f, "dropped"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerbStateKind {
    Captured,
    Active,
    Paused,
    Done,
    Dropped,
}

impl VerbState {
    fn kind(&self) -> VerbStateKind {
        match self {
            VerbState::Captured => VerbStateKind::Captured,
            VerbState::Active => VerbStateKind::Active,
            VerbState::Paused(_) => VerbStateKind::Paused,
            VerbState::Done => VerbStateKind::Done,
            VerbState::Dropped(_) => VerbStateKind::Dropped,
        }
    }
}

/// Value object describing *why* a verb was paused or dropped.
#[derive(Debug, Clone)]
pub struct Reason(String);

impl Reason {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(CoreError::EmptyReason)
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Domain event representing a valid state transition.
/// Returned for every successful state change.
/// MUST be persisted as an Action.
#[derive(Debug, Clone)]
pub struct StateChanged {
    pub verb_id: VerbId,
    pub from: VerbState,
    pub to: VerbState,
    pub at: OffsetDateTime,
}

impl StateChanged {
    pub fn new(verb_id: VerbId, from: VerbState, to: VerbState, at: OffsetDateTime) -> Self {
        Self {
            verb_id,
            from,
            to,
            at,
        }
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
