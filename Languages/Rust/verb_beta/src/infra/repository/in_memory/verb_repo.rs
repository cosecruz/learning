use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::{model::Verb, repository::VerbRepository};

/// In-memory verb repository
///
/// Implements the VerbRepository port using a Vec in memory.
#[derive(Clone)]
pub struct InMemoryVerbRepo {
    store: Arc<Mutex<Vec<Verb>>>,
}

impl InMemoryVerbRepo {
    pub fn new(store: Arc<Mutex<Vec<Verb>>>) -> Self {
        Self { store }
    }
}

impl VerbRepository for InMemoryVerbRepo {
    fn save(
        &self,
        verb: &Verb,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<(), crate::application::ApplicationError>> + Send + '_>,
    > {
        todo!()
    }

    fn find_by_id(
        &self,
        id: crate::domain::model::VerbId,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<Option<Verb>, crate::application::ApplicationError>>
                + Send
                + '_,
        >,
    > {
        todo!()
    }

    fn list(
        &self,
        filter: crate::domain::repository::verb_repo::VerbFilter,
    ) -> std::pin::Pin<
        Box<
            dyn Future<
                    Output = Result<
                        crate::domain::repository::verb_repo::VerbListResult,
                        crate::application::ApplicationError,
                    >,
                > + Send
                + '_,
        >,
    > {
        todo!()
    }
}
