use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::{model::ActionLog, repository::ActionLogRepository};

#[derive(Clone)]
pub struct InMemoryActionLogRepo {
    store: Arc<Mutex<Vec<ActionLog>>>,
}

impl InMemoryActionLogRepo {
    pub fn new(store: Arc<Mutex<Vec<ActionLog>>>) -> Self {
        Self { store }
    }
}

impl ActionLogRepository for InMemoryActionLogRepo {
    fn find_by_verb(
        &self,
        verb_id: crate::domain::model::VerbId,
        limit: usize,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<Vec<ActionLog>, crate::application::ApplicationError>>
                + Send
                + '_,
        >,
    > {
        todo!()
    }

    fn append(
        &self,
        log: &ActionLog,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<(), crate::application::ApplicationError>> + Send + '_>,
    > {
        todo!()
    }
}
