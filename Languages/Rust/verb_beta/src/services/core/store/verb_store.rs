use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::services::core::{
    error::CoreError,
    model::{
        action_log::{self, ActionLog},
        verb::Verb,
    },
};

/// Port: persistence for Verb aggregate
pub trait VerbStore: Send + Sync {
    fn save(&self, verb: &Verb) -> Result<(), CoreError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Verb>, CoreError>;
}

/// Port: persistence for ActionLog
pub trait ActionLogStore: Send + Sync {
    fn log(&self, action: &ActionLog) -> Result<(), CoreError>;
}

/// Port: transaction boundary
/// This allows infrastructure to decide how transactions work (DB, saga, event store, etc)
pub trait Transaction: Send {
    fn commit(self: Box<Self>) -> Result<(), CoreError>;
    fn rollback(self: Box<Self>) -> Result<(), CoreError>;
}

pub trait UnitOfWork: Send + Sync {
    fn begin(&self) -> Result<Box<dyn Transaction>, CoreError>;
}

// ---------------------------------------------------------------------------------------
// APPLICATION USE CASES
// ---------------------------------------------------------------------------------------

pub struct CreateVerb {
    verb_store: Arc<dyn VerbStore>,
    action_log_store: Arc<dyn ActionLogStore>,
    uow: Arc<dyn UnitOfWork>,
}

impl CreateVerb {
    // Constructor (DI-friendly)
    pub fn new(
        verb_store: Arc<dyn VerbStore>,
        action_log_store: Arc<dyn ActionLogStore>,
        uow: Arc<dyn UnitOfWork>,
    ) -> Self {
        Self {
            verb_store,
            action_log_store,
            uow,
        }
    }

    /// Application use case:
    /// Creates a verb and records its creation in the action log.
    ///
    /// Invariants:
    /// - Verb must be valid domain object
    /// - Creation and logging must be atomic
    /// - Domain rules are enforced by `Verb::new`
    pub fn execute(
        &self,
        title: impl Into<String>,
        description: Option<String>,
    ) -> Result<Uuid, CoreError> {
        let title = title.into();
        let description = description.unwrap_or_default();

        // begin transaction
        let tx = self.uow.begin()?;

        // domain logic
        let verb = Verb::new(title, description)?;
        let action_log = ActionLog::created(verb.id());

        // persistence
        self.verb_store.save(&verb)?;
        self.action_log_store.log(&action_log)?;

        // commit
        tx.commit()?;

        Ok(verb.id().as_uuid())
    }
}

//sql lite example
// pub struct SqliteUow {
//     pool: sqlx::SqlitePool,
// }

// pub struct SqliteTx {
//     tx: sqlx::Transaction<'static, sqlx::Sqlite>,
// }

// impl UnitOfWork for SqliteUow {
//     fn begin(&self) -> Result<Box<dyn Transaction>, CoreError> {
//         let tx = self.pool.begin().unwrap();
//         Ok(Box::new(SqliteTx { tx }))
//     }
// }

// impl Transaction for SqliteTx {
//     fn commit(self: Box<Self>) -> Result<(), CoreError> {
//         futures::executor::block_on(self.tx.commit())?;
//         Ok(())
//     }

//     fn rollback(self: Box<Self>) -> Result<(), CoreError> {
//         futures::executor::block_on(self.tx.rollback())?;
//         Ok(())
//     }
// }

// postgres example
// pub struct PgUow {
//     pool: sqlx::PgPool,
// }

// pub struct PgTx {
//     tx: sqlx::Transaction<'static, sqlx::Postgres>,
// }

// impl UnitOfWork for PgUow {
//     fn begin(&self) -> Result<Box<dyn Transaction>, CoreError> {
//         let tx = self.pool.begin().unwrap();
//         Ok(Box::new(PgTx { tx }))
//     }
// }

// impl Transaction for PgTx {
//     fn commit(self: Box<Self>) -> Result<(), CoreError> {
//         futures::executor::block_on(self.tx.commit())?;
//         Ok(())
//     }

//     fn rollback(self: Box<Self>) -> Result<(), CoreError> {
//         futures::executor::block_on(self.tx.rollback())?;
//         Ok(())
//     }
// }

pub struct MemoryUow;

pub struct MemoryTx;

impl UnitOfWork for MemoryUow {
    fn begin(&self) -> Result<Box<dyn Transaction>, CoreError> {
        Ok(Box::new(MemoryTx))
    }
}

impl Transaction for MemoryTx {
    fn commit(self: Box<Self>) -> Result<(), CoreError> {
        Ok(())
    }
    fn rollback(self: Box<Self>) -> Result<(), CoreError> {
        Ok(())
     }
}

// pub fn run_use_case() {
//     let db = Arc::new(Mutex::new(MemoryDb::default()));

//     let uow = Arc::new(MemoryUow { db: db.clone() });
//     let verb_store = Arc::new(MemoryVerbStore { db: db.clone() });
//     let log_store = Arc::new(MemoryActionLogStore { db: db.clone() });

//     let usecase = CreateVerb::new(verb_store, log_store, uow);
// }
