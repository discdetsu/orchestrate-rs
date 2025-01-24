use reqwest::Client;
use crate::serializers::{PredictionRequest, PredictionResponse};

pub struct Orchestrator {
    client: Client,
    sub_services: Vec<String>, // List of sub-service URLs
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            sub_services: vec![
                "http://sub_service_1:5001".to_string(),
                "http://sub_service_2:5002".to_string(),
                "http://sub_service_3:5003".to_string(),
            ],
        }
    }

    pub async fn process_requests(&self) -> Result<(), reqwest::Error> {
        for service_url in &self.sub_services {
            println!("Sending request to: {}", service_url);

            // Example request body
            let request_body = PredictionRequest {
                image_id: "12345".to_string(),
                payload: "example_payload".to_string(),
            };

            // Send POST request
            let response = self
                .client
                .post(service_url)
                .json(&request_body)
                .send()
                .await?;

            // Parse response
            let response_json: PredictionResponse = response.json().await?;
            println!("Response from {}: {:?}", service_url, response_json);
        }

        Ok(())
    }
}