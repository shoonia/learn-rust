use axum::{
    debug_handler,
    extract::OriginalUri,
    http::{Method, StatusCode},
    response::IntoResponse,
};

#[debug_handler]
pub async fn not_found_route(method: Method, OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("Route not found: {}: {}", method, uri),
    )
}
