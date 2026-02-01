// mod memory_db;
//trait db: contract that all dbs should implement
// pub trait Database {
//     type Pool;
//     type Error;

//     async fn connect(database_url: &str) -> Result<Self::Pool, Self::Error>;
//     async fn migrate(pool: &Self::Pool) -> Result<(), Self::Error>;
// }

mod builder;
mod database;
mod dyn_db; // ← NEW (type erasure layer)
mod factory;
mod singleton;

pub use builder::DatabaseBuilder;
pub use database::{Database, DatabaseTransaction};
pub use factory::DatabaseFactory;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Database type not supported: {0}")]
    NotSupported(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Query error: {0}")]
    Query(String),
}
