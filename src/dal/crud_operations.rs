use async_trait::async_trait;
use sqlx::{Error, SqlitePool};

#[async_trait]
pub trait CrudOperations<T> {
    async fn create(pool: &SqlitePool, items: &[T]) -> Result<(), Error>;
    async fn get(pool: &SqlitePool, ids: &[i64]) -> Result<Vec<T>, Error>;
    async fn list(pool: &SqlitePool) -> Result<Vec<T>, Error>;
    async fn update(pool: &SqlitePool, items: &[T]) -> Result<(), Error>;
    async fn delete(pool: &SqlitePool, ids: &[i64]) -> Result<(), Error>;
}
