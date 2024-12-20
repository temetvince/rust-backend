use async_trait::async_trait;
use sqlx::{query, query_as, Error, SqlitePool};

use super::{crud_operations::CrudOperations, models::Ware};

#[async_trait]
impl CrudOperations<Ware> for Ware {
    async fn create(pool: &SqlitePool, wares: &[Ware]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for ware in wares {
            validate_foreign_keys(
                pool,
                ware.group_id,
                ware.type_id,
            )
            .await?;

            query("INSERT INTO wares (name, volume, minimum, maximum, average, group_id, type_id) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(&ware.name)
                .bind(ware.volume)
                .bind(ware.minimum)
                .bind(ware.maximum)
                .bind(ware.average)
                .bind(ware.group_id)
                .bind(ware.type_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn get(pool: &SqlitePool, ids: &[i64]) -> Result<Vec<Ware>, Error> {
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let query_string = format!(
            "SELECT id, name, volume, minimum, maximum, average, group_id, type_id FROM wares WHERE id IN ({})",
            placeholders.join(", ")
        );

        let mut query = sqlx::query_as::<_, Ware>(&query_string);
        for id in ids {
            query = query.bind(id);
        }

        Ok(query.fetch_all(pool).await?)
    }

    async fn list(pool: &SqlitePool) -> Result<Vec<Ware>, Error> {
        let wares = query_as!(
            Ware,
            r#"
            SELECT id, name, volume, minimum, maximum, average, group_id, type_id
            FROM wares
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(wares)
    }

    async fn update(pool: &SqlitePool, wares: &[Ware]) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for ware in wares {
            validate_foreign_keys(
                pool,
                ware.group_id,
                ware.type_id,
            )
            .await?;

            query(
                "UPDATE wares SET name = ?, volume = ?, minimum = ?, maximum = ?, average = ?, group_id = ?, type_id = ? WHERE id = ?"
            )
            .bind(&ware.name)
            .bind(ware.volume)
            .bind(ware.minimum)
            .bind(ware.maximum)
            .bind(ware.average)
            .bind(ware.group_id)
            .bind(ware.type_id)
            .bind(ware.id)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn delete(pool: &SqlitePool, ids: &[i64]) -> Result<(), Error> {
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let query_string = format!(
            "DELETE FROM wares WHERE id IN ({})",
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&query_string);
        for id in ids {
            query = query.bind(id);
        }

        query.execute(pool).await?;
        Ok(())
    }
}

async fn validate_foreign_keys(
    pool: &SqlitePool,
    group_id: Option<i64>,
    type_id: Option<i64>,
) -> Result<(), Error> {
    if let Some(group_id) = group_id {
        let exists = query!(
            "SELECT COUNT(*) as count FROM groups WHERE id = ?",
            group_id
        )
        .fetch_one(pool)
        .await?;
        if exists.count == 0 {
            return Err(Error::RowNotFound);
        }
    }

    if let Some(type_id) = type_id {
        let exists = query!(
            "SELECT COUNT(*) as count FROM types WHERE id = ?",
            type_id
        )
        .fetch_one(pool)
        .await?;
        if exists.count == 0 {
            return Err(Error::RowNotFound);
        }
    }

    Ok(())
}
