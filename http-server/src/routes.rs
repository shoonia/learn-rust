use crate::model::{CalculationRequest, CalculationResult, Response};
use axum::{Json, debug_handler, extract::Query, http::StatusCode};

#[debug_handler]
pub async fn root() -> Json<Response> {
    Json(Response {
        message: "Hello, World!".to_string(),
    })
}

fn calculate(payload: CalculationRequest) -> Result<Json<CalculationResult>, StatusCode> {
    let result = match payload.operation.as_str() {
        "add" => payload.a + payload.b,
        "subtract" => payload.a - payload.b,
        "multiply" => payload.a * payload.b,
        "divide" => {
            if payload.b == 0.0 {
                return Err(StatusCode::BAD_REQUEST);
            } else {
                payload.a / payload.b
            }
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(CalculationResult { result }))
}

#[debug_handler]
pub async fn calculate_get(
    Query(payload): Query<CalculationRequest>,
) -> Result<Json<CalculationResult>, StatusCode> {
    calculate(payload)
}

#[debug_handler]
pub async fn calculate_post(
    Json(payload): Json<CalculationRequest>,
) -> Result<Json<CalculationResult>, StatusCode> {
    calculate(payload)
}
