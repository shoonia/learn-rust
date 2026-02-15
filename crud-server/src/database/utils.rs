use axum::http::StatusCode;
use sqlx::Error;

pub fn map_error(error: Error) -> (StatusCode, String) {
    match error {
        Error::RowNotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
        Error::InvalidArgument(name) => match name.as_str() {
            "revision" => (StatusCode::CONFLICT, "Revision mismatch".to_string()),
            _ => (
                StatusCode::BAD_REQUEST,
                format!("Invalid argument: {}", name),
            ),
        },
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unexpected error: {}", error),
        ),
    }
}
