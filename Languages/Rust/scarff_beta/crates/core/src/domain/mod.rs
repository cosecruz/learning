pub mod errors;
pub mod target;

pub use errors::DomainError;
pub use target::{
    Architecture, Framework, Language, ProjectType, PythonFramework, RustFramework, Target,
    TypeScriptFramework,
};

// template
#[derive(Debug, Clone)]
pub struct Template {}

// project_structure
#[derive(Debug, Clone)]
pub struct ProjectStructure {}
