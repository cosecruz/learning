// use std::future::Future;
// use std::pin::Pin;

// use super::{Database, DatabaseError, DatabaseTransaction};
// use crate::domain::{
//     error::DomainError,
//     repository::{ActionLogRepository, VerbRepository},
// };

// /* ================================
//    Erased repository traits
// ================================ */
// pub trait DynVerbRepository: Send + Sync {
//     fn save(&self, verb: &crate::domain::model::Verb) -> Result<(), DomainError>;

//     fn find_by_id(
//         &self,
//         id: crate::domain::model::VerbId,
//     ) -> Result<Option<crate::domain::model::Verb>, DomainError>;

//     fn list(
//         &self,
//         filter: crate::domain::repository::verb_repo::VerbFilter,
//     ) -> Result<crate::domain::repository::verb_repo::VerbListResult, DomainError>;
// }

// pub trait DynActionLogRepository: Send + Sync {
//     fn append(&self, log: &crate::domain::model::ActionLog) -> Result<(), DomainError>;

//     fn find_by_verb(
//         &self,
//         verb_id: crate::domain::model::VerbId,
//         limit: usize,
//     ) -> Result<Vec<crate::domain::model::ActionLog>, DomainError>;
// }

// /* ================================
//    Erased DB traits
// ================================ */
// pub trait DynDatabase: Send + Sync {
//     fn begin_tx(
//         &self,
//     ) -> Pin<Box<dyn Future<Output = Result<Box<dyn DynTransaction>, DatabaseError>> + Send>>;
// }

// pub trait DynTransaction: Send + Sync {
//     fn verb_repository(&self) -> &dyn DynVerbRepository;
//     fn action_log_repository(&self) -> &dyn DynActionLogRepository;

//     fn commit(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send>>;
// }

// /* ================================
//    Bridges (generic → erased)
// ================================ */
// impl<T> DynDatabase for T
// where
//     T: Database + Send + Sync + 'static,
//     T::Tx: DatabaseTransaction + Send + Sync + 'static,
// {
//     fn begin_tx(
//         &self,
//     ) -> Pin<Box<dyn Future<Output = Result<Box<dyn DynTransaction>, DatabaseError>> + Send>> {
//         Box::pin(async move {
//             let tx = self.begin_tx().await?;
//             Ok(Box::new(tx) as Box<dyn DynTransaction>)
//         })
//     }
// }

// impl<T> DynTransaction for T
// where
//     T: DatabaseTransaction + Send + Sync + 'static,
//     T::VerbRepo: VerbRepository<Error = DomainError> + 'static,
//     T::ActionLogRepo: ActionLogRepository<Error = DomainError> + 'static,
// {
//     fn verb_repository(&self) -> &dyn DynVerbRepository {
//         self.verb_repository()
//     }

//     fn action_log_repository(&self) -> &dyn DynActionLogRepository {
//         self.action_log_repository()
//     }

//     fn commit(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<(), DatabaseError>> + Send>> {
//         Box::pin(async move { (*self).commit().await })
//     }
// }

// /* ================================
//    Repo bridges
// ================================ */
// impl<T> DynVerbRepository for T
// where
//     T: VerbRepository<Error = DomainError> + Send + Sync,
// {
//     fn save(&self, verb: &crate::domain::model::Verb) -> Result<(), DomainError> {
//         VerbRepository::save(self, verb)
//     }

//     fn find_by_id(
//         &self,
//         id: crate::domain::model::VerbId,
//     ) -> Result<Option<crate::domain::model::Verb>, DomainError> {
//         VerbRepository::find_by_id(self, id)
//     }

//     fn list(
//         &self,
//         filter: crate::domain::repository::verb_repo::VerbFilter,
//     ) -> Result<crate::domain::repository::verb_repo::VerbListResult, DomainError> {
//         VerbRepository::list(self, filter)
//     }
// }

// impl<T> DynActionLogRepository for T
// where
//     T: ActionLogRepository<Error = DomainError> + Send + Sync,
// {
//     fn append(&self, log: &crate::domain::model::ActionLog) -> Result<(), DomainError> {
//         ActionLogRepository::append(self, log)
//     }

//     fn find_by_verb(
//         &self,
//         verb_id: crate::domain::model::VerbId,
//         limit: usize,
//     ) -> Result<Vec<crate::domain::model::ActionLog>, DomainError> {
//         ActionLogRepository::find_by_verb(self, verb_id, limit)
//     }
// }
