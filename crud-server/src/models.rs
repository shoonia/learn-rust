use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize, Validate)]
pub struct Task {
    #[validate(range(min = 0, message = "`id` must be a non-negative integer"))]
    pub id: i64,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`name` must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`details` must be between 1 and 255 characters"
    ))]
    pub details: String,
}

#[derive(Serialize)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "`name` must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`details` must be between 1 and 255 characters"
    ))]
    pub details: String,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}
