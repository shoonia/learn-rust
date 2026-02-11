use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
    pub operation: String,
}

#[derive(Serialize)]
pub struct CalculationResult {
    pub result: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MyApiResponse {
    pub origin: String,
}
