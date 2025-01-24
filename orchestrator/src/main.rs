mod orchestrator;
mod serializers;

use orchestrator::Orchestrator;
use tokio;

#[tokio::main]
async fn main() {
    let orchestrator = Orchestrator::new();

    match orchestrator.process_requests().await {
        Ok(_) => println!("All requests processed successfully."),
        Err(e) => eprintln!("Error processing requests: {}", e),
    }
}