use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, FromRow)]
pub struct Ware {
    pub id: Option<i64>,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(range(min = 0, message = "Must be non-negative"))]
    pub volume: i64,

    #[validate(range(min = 1, message = "Must be positive"))]
    pub minimum: i64,

    #[validate(range(min = 1, message = "Must be positive"))]
    pub maximum: i64,

    #[validate(range(min = 1, message = "Must be positive"))]
    pub average: i64,

    #[validate(range(min = 1, message = "Must be positive"))]
    pub group_id: Option<i64>,

    #[validate(range(min = 1, message = "Must be positive"))]
    pub type_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromRow)]
pub struct Group {
    pub id: Option<i64>,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromRow)]
pub struct Type {
    pub id: Option<i64>,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}
