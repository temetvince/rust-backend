use async_trait::async_trait;
use sqlx::{query, query_as, Error, SqlitePool};

use super::{crud_operations::CrudOperations, models::Group};

#[async_trait]
impl CrudOperations<Group> for Group {
    async fn create(pool: &SqlitePool, groups: &[Group]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for group in groups {
            query("INSERT INTO groups (name) VALUES (?)")
                .bind(&group.name)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn get(pool: &SqlitePool, ids: &[i64]) -> Result<Vec<Group>, Error> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
    
        let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query_string = format!("SELECT id, name FROM groups WHERE id IN ({})", placeholders);
    
        let mut query = sqlx::query_as::<_, Group>(&query_string);
        for id in ids {
            query = query.bind(id);
        }
    
        let groups = query.fetch_all(pool).await?;
        Ok(groups)
    }

    async fn list(pool: &SqlitePool) -> Result<Vec<Group>, Error> {
        let groups = query_as!(Group, "SELECT id, name FROM groups")
            .fetch_all(pool)
            .await?;
        Ok(groups)
    }

    async fn update(pool: &SqlitePool, groups: &[Group]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for group in groups {
            query("UPDATE groups SET name = ? WHERE id = ?")
                .bind(&group.name)
                .bind(group.id)
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
        let query_string = format!("DELETE FROM groups WHERE id IN ({})", placeholders);
    
        let mut query = sqlx::query(&query_string);
        for id in ids {
            query = query.bind(id);
        }
    
        query.execute(pool).await?;
        Ok(())
    }
}
