use crate::domain::{
    error::DomainError,
    model::{ActionLog, Verb, VerbId, VerbState},
};

// ==================================================
// VERB REPOSITORY TRAIT
// ==================================================
pub trait VerbRepository: Send + Sync {
    ///Create a new verb and its initial action log atomically
    async fn create(&self, verb: &Verb, action_log: &ActionLog) -> Result<(), DomainError>;

    /// Get verb by ID
    async fn get_by_id(&self, id: VerbId) -> Result<Option<Verb>, DomainError>;

    /// List verbs with optional filtering
    async fn list(&self, filter: VerbFilter) -> Result<VerbListResult, DomainError>;

    /// Update verb state and append action log atomically
    async fn update_state(&self, verb: &Verb, action_log: &ActionLog) -> Result<(), DomainError>;
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
