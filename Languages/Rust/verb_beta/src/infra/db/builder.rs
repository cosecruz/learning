use super::{Database, DatabaseError, DatabaseFactory};

/// Builder pattern for database configuration
///
/// Usage:
/// ```
/// let db = DatabaseBuilder::new()
///     .sqlite("app.db")
///     .build()
///     .await?;
/// ```
pub struct DatabaseBuilder {
    config: DatabaseConfig,
}

#[derive(Debug, Clone)]
enum DatabaseConfig {
    InMemory,
    // Sqlite(String),
}

impl DatabaseConfig {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseConfig::InMemory => "in_memory",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DatabaseError> {
        match s {
            "in_memory" => Ok(Self::InMemory),
            _ => return Err(DatabaseError::InvalidConfig(s.into())),
        }
    }
}

impl DatabaseBuilder {
    /// Use in-memory database (for testing)
    pub fn in_memory(mut self) -> Self {
        self.config = DatabaseConfig::InMemory;
        self
    }

    // Use SQLite database
    // pub fn sqlite(mut self, path: impl Into<String>) -> Self {
    //     self.config = DatabaseConfig::Sqlite(path.into());
    //     self
    // }

    /// Build the database
    pub async fn build(self) -> Result<InMemoryDatabase, DatabaseError> {
        match self.config {
            DatabaseConfig::InMemory => DatabaseFactory::create_in_memory().await,
        }
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
