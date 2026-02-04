pub mod errors;
pub mod target;

pub use errors::DomainError;
pub use target::{Language, Target};

// template
#[derive(Debug, Clone)]
pub struct Template {}

// project_structure
#[derive(Debug, Clone)]
pub struct ProjectStructure {}
