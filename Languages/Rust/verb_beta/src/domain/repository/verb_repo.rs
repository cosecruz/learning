use std::pin::Pin;

use crate::{
    application::ApplicationError,
    domain::model::{Verb, VerbId, VerbState},
};

// ==================================================
// VERB REPOSITORY TRAIT
// ==================================================
/// PORT: What the domain needs from verb persistence
///
/// ## Why Pin<Box<dyn Future>>?
/// - Domain layer is agnostic to async implementation
/// - Repositories are used behind `&dyn VerbRepository`
/// - `async fn` would make trait NOT object-safe
/// - Boxed futures maintain object safety
pub trait VerbRepository: Send + Sync {
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
    pub total: u32,
}
