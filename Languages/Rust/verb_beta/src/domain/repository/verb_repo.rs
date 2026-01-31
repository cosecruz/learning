use std::pin::Pin;

use crate::{
    application::ApplicationError,
    domain::model::{ActionLog, Verb, VerbId, VerbState},
};

// ==================================================
// VERB REPOSITORY TRAIT
// ==================================================
/// PORT: What the domain needs from persistence.
///
/// This is a **port** in hexagonal architecture.
/// It defines capabilities without knowing how they're implemented.
///
/// Note: This is NOT async because the domain layer is sync.
/// The application layer will use async trait implementations.
pub trait VerbRepository: Send + Sync + 'static {
    /// Store a verb
    fn save(
        &self,
        verb: &Verb,
    ) -> Pin<Box<dyn Future<Output = Result<(), ApplicationError>> + Send + '_>>;

    /// Retrieve verb by ID
    fn find_by_id(
        &self,
        id: VerbId,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Verb>, ApplicationError>> + Send + '_>>;

    /// List verbs with optional filtering
    fn list(
        &self,
        filter: VerbFilter,
    ) -> Pin<Box<dyn Future<Output = Result<VerbListResult, ApplicationError>> + Send + '_>>;

    // Update verb state and append action log atomically
    // fn update_state(&self, verb: &Verb, action_log: &ActionLog) -> Result<(), Self::Error>;
}

// ============================================================================
// FILTER & RESULT TYPES
// ============================================================================

#[derive(Debug, Clone)]
pub struct VerbFilter {
    pub state: Option<VerbState>,
    pub limit: u32,
    pub offset: u32,
}

impl Default for VerbFilter {
    fn default() -> Self {
        Self {
            state: None,
            limit: 50,
            offset: 0,
        }
    }
}

impl VerbFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(mut self, state: VerbState) -> Self {
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
pub struct VerbListResult {
    pub verbs: Vec<Verb>,
    pub total: i64,
}
