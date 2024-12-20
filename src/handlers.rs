use axum::{Router, Json, extract::{Path, State}, http::StatusCode};
use crate::db;
use crate::models::User;
use sqlx::SqlitePool;
use serde_json::json;
use validator::Validate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
        };
        (status, Json(json!({ "error": body }))).into_response()
    }
}

pub fn user_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/users", axum::routing::get(list_users).post(create_user))
        .route("/users/:id", axum::routing::get(get_user).put(update_user).delete(delete_user))
        .with_state(pool)
}

async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<User>,
) -> Result<StatusCode, ApiError> {
    payload.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    db::create_user(&pool, &payload).await?;
    Ok(StatusCode::CREATED)
}

async fn get_user(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    let user = db::get_user(&pool, user_id).await?;
    Ok(Json(user))
}

async fn list_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = db::list_users(&pool).await?;
    Ok(Json(users))
}

async fn update_user(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
    Json(mut payload): Json<User>,
) -> Result<StatusCode, ApiError> {
    payload.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    payload.id = Some(user_id);
    db::update_user(&pool, &payload).await?;
    Ok(StatusCode::OK)
}

async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    db::delete_user(&pool, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
