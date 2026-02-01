use std::{fmt, pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::sync::OnceCell;

use crate::infra::repository::in_memory::InMemoryDatabase;

use super::{Database, DatabaseError};

/// Factory for creating database instances
///
/// Encapsulates creation logic for different database backends.
pub struct DatabaseFactory;

impl DatabaseFactory {
    /// Create an in-memory database
    pub async fn create_in_memory() -> Result<InMemoryDatabase, DatabaseError> {
        Ok(InMemoryDatabase::new())
    }

    // Uncomment when SQLite is implemented
    // pub async fn create_sqlite(path: &str) -> Result<SqliteDatabase, DatabaseError> {
    //     SqliteDatabase::new(path).await
    // }
}
