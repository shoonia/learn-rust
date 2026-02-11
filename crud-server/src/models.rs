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

#[derive(Deserialize, Validate)]
pub struct ListRequest {
    #[validate(range(min = 1, max = 100, message = "`limit` must be between 1 and 100"))]
    pub limit: Option<u32>,

    #[validate(range(min = 1, message = "`offset` must be greater than or equal to 1"))]
    pub offset: Option<u32>,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub tasks: Vec<Task>,
}
