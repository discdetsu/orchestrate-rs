use serde::{Deserialize, Serialize};

// Request structure (if needed for sub-services)
#[derive(Serialize)]
pub struct PredictionRequest {
    pub image_id: String,
    pub payload: String,
}

// Response structure for mock services
#[derive(Deserialize, Debug)]
pub struct PredictionResponse {
    pub status_code: u16,
    pub app_code: String,
    pub message: String,
    pub data: PredictionData,
}

#[derive(Deserialize, Debug)]
pub struct PredictionData {
    pub api_version: String,
    pub service_name: String,
    pub prediction: PredictionDetails,
}

#[derive(Deserialize, Debug)]
pub struct PredictionDetails {
    pub image: String,
    pub json: serde_json::Value, // Dynamic structure for predictions
}