# Orchestrator Service

## Overview
This project is a Rust-based web service built with Actix Web. It provides an endpoint to trigger processing in an `Orchestrator` module, which manages requests for sub-services.

## Features
- Actix Web-based HTTP server
- Thread-safe request orchestration using `Arc`
- JSON responses for success and error handling

## Installation
### Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust installation)

### Clone the Repository
```sh
git clone <repository-url>
cd <repository-name>
```

### Build the Project
```sh
cargo build --release
```

## Running the Server
```sh
cargo run
```
The server will start at `http://127.0.0.1:8080`.

## API Endpoints
### Trigger Processing
**Endpoint:** `POST /trigger`

**Description:** Triggers the orchestrator to process requests for sub-services.

**Response Format:**
- **Success (200 OK)**
  ```json
  {
    "status": "success",
    "message": "All sub-services processed successfully"
  }
  ```
- **Error (500 Internal Server Error)**
  ```json
  {
    "status": "error",
    "message": "Error processing requests: <error-message>"
  }
  ```

## Project Structure
```
.
├── src
│   ├── main.rs           # Entry point of the application
│   ├── orchestrator.rs   # Orchestrator module handling request processing
│   ├── serializers.rs    # Defines response serialization structures
├── Cargo.toml            # Dependencies and project metadata
└── README.md             # Documentation
```

## License
This project is licensed under the MIT License.

