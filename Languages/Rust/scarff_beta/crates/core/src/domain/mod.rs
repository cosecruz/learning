pub mod errors;
pub mod project_structure;
pub mod target;
pub mod template;

pub use errors::DomainError;
pub use target::{
    Architecture, Framework, Language, ProjectType, PythonFramework, RustFramework, Target,
    TypeScriptFramework,
};
