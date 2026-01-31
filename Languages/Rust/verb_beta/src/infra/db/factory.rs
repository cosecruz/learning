use std::{fmt, pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::sync::OnceCell;

use crate::infra::repository::in_memory::InMemoryDatabase;

use super::{Database, DatabaseError};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum DatabaseType {
    InMemory,
    Sqlite,
    Postgres,
}

impl DatabaseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::InMemory => "in_memory",
            DatabaseType::Sqlite => "sqlite",
            DatabaseType::Postgres => "postgres",
        }
    }
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Factory: Creates concrete database implementations
///
/// The factory pattern encapsulates the logic of creating
/// different database backends.
pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create_database(db_type: DatabaseType) -> Result<Box<dyn Builder>, DatabaseError> {
        match db_type {
            DatabaseType::InMemory => Ok(InMemoryBuilder),
            _ => return Err(DatabaseError::NotSupported(db_type.to_string())),
        }
    }
}

pub trait Builder: Send + 'static {
    type DB<'a>: Database + 'a
    where
        Self: 'a;

    fn build<'a>(&self) -> Pin<Box<dyn Future<Output = Result<Self::DB<'a>, DatabaseError>>>>;
}

struct InMemoryBuilder;

impl Builder for InMemoryBuilder {
    type DB<'a>
        = InMemoryDatabase
    where
        Self: 'a;

    fn build<'a>(&self) -> Pin<Box<dyn Future<Output = Result<Self::DB<'a>, DatabaseError>>>> {
        Box::pin(async move { Ok(InMemoryDatabase::new()) })
    }
}

pub static DATABASE: OnceCell<Arc<dyn Database>> = OnceCell::new();

// flow
// logic: you build in the factory so the factory infact has the builder or returns the builder
// factory: creates specific database
// builder: builds database from config should return a singleton database
// singleton: a singular instance database globally
