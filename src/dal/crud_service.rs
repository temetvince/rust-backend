use sqlx::sqlite::SqlitePool;
use tokio::sync::OnceCell;

use crate::dal::crud_operations::CrudOperations;

pub struct CrudService {
    pool: SqlitePool,
}

type SharedConnection = SqlitePool;
static DATABASE_URL: &str = "sqlite://DB.db";
static DB_POOL: OnceCell<SharedConnection> = OnceCell::const_new();

impl CrudService {
    /// Creates a new instance of `CrudService`.
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(DATABASE_URL)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to apply migrations");

        DB_POOL
            .set(pool.clone())
            .expect("Failed to set DB pool");

        CrudService { pool }
    }

    /// Generic method to create items.
    pub async fn create<T>(&self, items: &[T]) -> Result<(), sqlx::Error>
    where
        T: CrudOperations<T> + Sync + Send,
    {
        T::create(&self.pool, items).await
    }

    /// Generic method to get items by IDs.
    pub async fn get<T>(&self, ids: &[i64]) -> Result<Vec<T>, sqlx::Error>
    where
        T: CrudOperations<T> + Sync + Send,
    {
        T::get(&self.pool, ids).await
    }

    /// Generic method to list all items.
    pub async fn list<T>(&self) -> Result<Vec<T>, sqlx::Error>
    where
        T: CrudOperations<T> + Sync + Send,
    {
        T::list(&self.pool).await
    }

    /// Generic method to update items.
    pub async fn update<T>(&self, items: &[T]) -> Result<(), sqlx::Error>
    where
        T: CrudOperations<T> + Sync + Send,
    {
        T::update(&self.pool, items).await
    }

    /// Generic method to delete items by IDs.
    pub async fn delete<T>(&self, ids: &[i64]) -> Result<(), sqlx::Error>
    where
        T: CrudOperations<T> + Sync + Send,
    {
        T::delete(&self.pool, ids).await
    }
}
