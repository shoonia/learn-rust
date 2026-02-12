use axum::{
    debug_handler,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};

#[debug_handler]
pub async fn not_found_route(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("Route not found: {}: {}", method, uri),
    )
}
