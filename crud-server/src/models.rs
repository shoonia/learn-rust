use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize, Validate)]
pub struct Task {
    #[validate(range(min = 1, message = "`id` must be a positive integer"))]
    pub id: i64,

    #[validate(range(min = 1, message = "`revision` must be a positive integer"))]
    pub revision: i64,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`title` must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`details` must be between 1 and 255 characters"
    ))]
    pub details: String,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
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
        message = "`title` must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "`details` must be between 1 and 255 characters"
    ))]
    pub details: String,
}

#[derive(Deserialize, Validate)]
pub struct DeleteRequest {
    #[validate(range(min = 1, message = "`id` must be a positive integer"))]
    pub id: i64,
    #[validate(range(min = 1, message = "`revision` must be a positive integer"))]
    pub revision: i64,
}

#[derive(Deserialize, Validate)]
pub struct ListRequest {
    #[validate(range(min = 1, max = 100, message = "`limit` must be between 1 and 100"))]
    pub limit: Option<u32>,

    #[validate(range(min = 0, message = "`offset` must be greater than or equal to 0"))]
    pub offset: Option<u32>,
}

#[derive(Serialize)]
pub struct Pagin {
    pub total: i64,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub tasks: Vec<Task>,
    pub pagin: Pagin,
}
