use async_trait::async_trait;
use sqlx::{query, query_as, Error, SqlitePool};

use super::{crud_operations::CrudOperations, models::Type};

#[async_trait]
impl CrudOperations<Type> for Type {
    async fn create(pool: &SqlitePool, types: &[Type]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for type_data in types {
            query("INSERT INTO types (name) VALUES (?)")
                .bind(&type_data.name)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn get(pool: &SqlitePool, ids: &[i64]) -> Result<Vec<Type>, Error> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
    
        let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query_string = format!("SELECT id, name FROM types WHERE id IN ({})", placeholders);
    
        let mut query = sqlx::query_as::<_, Type>(&query_string);
        for id in ids {
            query = query.bind(id);
        }
    
        let groups = query.fetch_all(pool).await?;
        Ok(groups)
    }

    async fn list(pool: &SqlitePool) -> Result<Vec<Type>, Error> {
        let types = query_as!(Type, "SELECT id, name FROM types")
            .fetch_all(pool)
            .await?;
        Ok(types)
    }

    async fn update(pool: &SqlitePool, types: &[Type]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for type_data in types {
            query("UPDATE types SET name = ? WHERE id = ?")
                .bind(&type_data.name)
                .bind(type_data.id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn delete(pool: &SqlitePool, ids: &[i64]) -> Result<(), Error> {
        if ids.is_empty() {
            return Ok(());
        }
    
        let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query_string = format!("DELETE FROM types WHERE id IN ({})", placeholders);
    
        let mut query = sqlx::query(&query_string);
        for id in ids {
            query = query.bind(id);
        }
    
        query.execute(pool).await?;
        Ok(())
    }
    
}
