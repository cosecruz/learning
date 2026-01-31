//! Repository :
//! a kind of abstraction layer to help define application
//! use cases as well as facilitate persistence
pub mod action_log_repo;
pub mod verb_repo;

pub use action_log_repo::ActionLogRepository;
pub use verb_repo::VerbRepository;
