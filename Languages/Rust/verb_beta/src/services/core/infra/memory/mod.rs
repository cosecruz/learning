pub mod action_log_store;
pub mod uow;
pub mod verb_store;

// Interface (axum)
//    ↓
// Application (usecases)
//    ↓
// Domain (entities + rules)
//    ↓
// Ports (traits)
//    ↓
// Infrastructure (memory / sqlite / postgres / redis / kafka / http)
