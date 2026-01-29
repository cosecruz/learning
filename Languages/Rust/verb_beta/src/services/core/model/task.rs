use crate::services::core::model::verb::VerbId;

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
