mod memory_db;
///trait db: contract that all dbs should implement
pub trait Database {
    type Pool;
    type Error;

    async fn connect(database_url: &str) -> Result<Self::Pool, Self::Error>;
    async fn migrate(pool: &Self::Pool) -> Result<(), Self::Error>;
}
