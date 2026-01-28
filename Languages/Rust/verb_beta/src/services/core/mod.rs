//! ### Core
//!
//! The `core` service contains the minimum business capabilities required
//! for the MVP and for architectural exploration.
//!
//! Supported use cases:
//!
//! - Create a verb (description only)
//! - List all verbs
//! - View a single verb
//! - Update verb state (`Captured → Active → Done`)
//! - Drop a verb (with optional reason)
//! - View the action log for a verb
//!
use core::fmt;

use time::OffsetDateTime;

pub(super) mod error;
use error::CoreError;

/// Strongly-typed identity for a Verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerbId(pub u64);

/// Domain entity representing a user's intent over time.
#[derive(Debug, Clone)]
pub struct Verb {
    id: VerbId,
    title: String,
    description: String,
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
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            state: VerbState::Captured,
            created_at: now,
            updated_at: now,
            context: None,
        }
    }

    pub fn id(&self) -> VerbId {
        self.id
    }

    pub fn state(&self) -> &VerbState {
        &self.state
    }

    // pub fn activate(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
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
    // ) -> Result<StateTransition, CoreError> {
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

    // pub fn complete(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
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
    // ) -> Result<StateTransition, CoreError> {
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

    // pub fn reopen(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
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
    ) -> Result<StateTransition, CoreError> {
        let from = self.state.clone();
        self.state = next.clone();
        self.updated_at = now;

        Ok(StateTransition::new(self.id, from, next, now))
    }
}

/// Transition abstraction to check if state transition is valid
impl Verb {
    fn ensure_can_transition_to(&self, next: &VerbState) -> Result<(), CoreError> {
        let from = self.state.clone();
        let to = next.clone();

        let allowed = match (&self.state, next) {
            // Captured | Paused → Active
            (VerbState::Captured, VerbState::Active) => true,
            (VerbState::Paused(_), VerbState::Active) => true,

            // Active → Paused
            (VerbState::Active, VerbState::Paused(_)) => true,

            // Active → Done
            (VerbState::Active, VerbState::Done) => true,

            // Done → Active
            (VerbState::Done, VerbState::Active) => true,

            // Any except Done → Dropped
            (VerbState::Done, VerbState::Dropped(_)) => false,
            (_, VerbState::Dropped(_)) => true,

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
    pub fn activate(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
        self.apply_transition(VerbState::Active, now)
    }

    /// Transition: Active → Paused(reason)
    pub fn pause(
        &mut self,
        reason: Reason,
        now: OffsetDateTime,
    ) -> Result<StateTransition, CoreError> {
        self.apply_transition(VerbState::Paused(reason), now)
    }

    /// Transition: Active → Done
    pub fn complete(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
        self.apply_transition(VerbState::Done, now)
    }

    /// Transition: Any (except Done) → Dropped(reason)
    pub fn drop(
        &mut self,
        reason: Reason,
        now: OffsetDateTime,
    ) -> Result<StateTransition, CoreError> {
        self.apply_transition(VerbState::Dropped(reason), now)
    }

    /// Re-open a completed verb.
    /// Transition: Done → Active
    pub fn reopen(&mut self, now: OffsetDateTime) -> Result<StateTransition, CoreError> {
        self.apply_transition(VerbState::Active, now)
    }
}

///apply transition to state if it is valid
impl Verb {
    fn apply_transition(
        &mut self,
        next: VerbState,
        now: OffsetDateTime,
    ) -> Result<StateTransition, CoreError> {
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
///Now this states are designed transition in a specific to be valid
/// Captured -> Active -> Done -want to reopen-> Active
///             |
///         Paused| Dropped
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

/// Value object describing *why* a verb was paused or dropped.
#[derive(Debug, Clone)]
pub struct Reason(String);

impl Reason {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(CoreError::InvalidReason)
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Domain event representing a valid state transition.
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub verb_id: VerbId,
    pub from: VerbState,
    pub to: VerbState,
    pub at: OffsetDateTime,
}

impl StateTransition {
    pub fn new(verb_id: VerbId, from: VerbState, to: VerbState, at: OffsetDateTime) -> Self {
        Self {
            verb_id,
            from,
            to,
            at,
        }
    }
}

/* ============================
Tasks
============================ */
///A concrete execution step within a verb.
///Tasks are optional initially. They emerge when a verb is too large to track as a single action.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

#[derive(Debug, Clone)]
pub struct Task {
    id: TaskId,
    verb_id: VerbId,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(id: TaskId, verb_id: VerbId, description: impl Into<String>) -> Self {
        Self {
            id,
            verb_id,
            description: description.into(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

/// A discrete, immutable fact describing something that happened in the system.
#[derive(Debug, Clone)]
pub enum _Action {
    Created {
        verb_id: VerbId,
        at: OffsetDateTime,
    },
    Activated {
        verb_id: VerbId,
        at: OffsetDateTime,
    },
    Paused {
        verb_id: VerbId,
        reason: Reason,
        at: OffsetDateTime,
    },
    Completed {
        verb_id: VerbId,
        at: OffsetDateTime,
    },
    Dropped {
        verb_id: VerbId,
        reason: Reason,
        at: OffsetDateTime,
    },
}
