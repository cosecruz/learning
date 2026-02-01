use crate::infra::repository::in_memory::InMemoryDatabase;

use super::{Database, DatabaseError, DatabaseFactory};

/// Builder pattern for database configuration
///
/// Provides a fluent API for constructing database instances.
pub struct DatabaseBuilder {
    config: DatabaseConfig,
}

#[derive(Debug, Clone)]
enum DatabaseConfig {
    InMemory,
    // Sqlite(String),  // Commented out for MVP
}

impl DatabaseBuilder {
    /// Create a new builder with default (in-memory) configuration
    pub fn new() -> Self {
        Self {
            config: DatabaseConfig::InMemory,
        }
    }

    /// Configure for in-memory database (for testing)
    pub fn in_memory(mut self) -> Self {
        self.config = DatabaseConfig::InMemory;
        self
    }

    // Uncomment when SQLite is implemented
    // pub fn sqlite(mut self, path: impl Into<String>) -> Self {
    //     self.config = DatabaseConfig::Sqlite(path.into());
    //     self
    // }

    /// Build the configured database
    ///
    /// Returns a concrete InMemoryDatabase for now.
    /// When multiple backends are added, return `Box<dyn Database>` with type erasure.
    pub async fn build(self) -> Result<InMemoryDatabase, DatabaseError> {
        match self.config {
            DatabaseConfig::InMemory => DatabaseFactory::create_in_memory().await,
            // DatabaseConfig::Sqlite(path) => DatabaseFactory::create_sqlite(&path).await,
        }
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
