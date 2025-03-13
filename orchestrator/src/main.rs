// main.rs
mod orchestrator;
mod serializers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use orchestrator::Orchestrator;
use serializers::TriggerResponse;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orchestrator = Arc::new(Orchestrator::new());
    
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(orchestrator.clone()))
            .service(
                web::resource("/trigger")
                    .route(web::post().to(trigger_processing))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn trigger_processing(
    orchestrator: web::Data<Arc<Orchestrator>>
) -> impl Responder {
    match orchestrator.process_requests().await {
        Ok(_) => {
            let response = TriggerResponse {
                status: "success".to_string(),
                message: "All sub-services processed successfully".to_string(),
            };
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = TriggerResponse {
                status: "error".to_string(),
                message: format!("Error processing requests: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}