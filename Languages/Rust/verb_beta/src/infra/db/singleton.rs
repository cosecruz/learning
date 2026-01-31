use std::sync::Arc;

use tokio::sync::OnceCell;

use super::Database;

/// Singleton: Global database instance
///
/// This ensures a single shared database connection pool
/// across the entire application.
///
/// Usage:
/// ```
/// DATABASE.set(db).expect("Database already initialized");
/// let db = DATABASE.get().expect("Database not initialized");
/// ```
pub static DATABASE: OnceCell<Arc<dyn Database<Tx = impl super::DatabaseTransaction>>> =
    OnceCell::new();

/// Initialize the global database
pub fn init_database<D: Database + 'static>(db: D) -> Result<(), String> {
    DATABASE
        .set(Arc::new(db))
        .map_err(|_| "Database already initialized".to_string())
}

/// Get the global database
pub fn get_database() -> Option<&'static Arc<dyn Database<Tx = impl super::DatabaseTransaction>>> {
    DATABASE.get()
}
