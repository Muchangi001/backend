use axum::{
    extract::Json as ExtractJson,
    http::{HeaderValue, Method, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u8,
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
    email: String,
    age: u8,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    })
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Server is alive and kicking!")
}

async fn create_user(ExtractJson(payload): ExtractJson<CreateUserRequest>) -> (StatusCode, Json<serde_json::Value>) {
    if payload.name.trim().is_empty() {
        let error_response = ErrorResponse {
            error: "Validation Error".to_string(),
            message: "Name cannot be empty".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(serde_json::to_value(error_response).unwrap()));
    }

    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        let error_response = ErrorResponse {
            error: "Validation Error".to_string(),
            message: "Please provide a valid email address".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(serde_json::to_value(error_response).unwrap()));
    }

    if payload.age == 0 {
        let error_response = ErrorResponse {
            error: "Validation Error".to_string(),
            message: "Age must be greater than 0".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(serde_json::to_value(error_response).unwrap()));
    }

    let fake_user_id = 42;
    
    println!("Creating user: {} ({})", payload.name, payload.email);
    
    let response = CreateUserResponse {
        id: fake_user_id,
        name: payload.name.trim().to_string(),
        email: payload.email.trim().to_string(),
        age: payload.age,
        message: "User created successfully!".to_string(),
    };
    
    (StatusCode::CREATED, Json(serde_json::to_value(response).unwrap()))
}

async fn get_users() -> Json<Vec<serde_json::Value>> {
    let fake_users = vec![
        serde_json::json!({
            "id": 1,
            "name": "Alice",
            "email": "alice@example.com",
            "age": 25
        }),
        serde_json::json!({
            "id": 2,
            "name": "Bob",
            "email": "bob@example.com",
            "age": 30
        }),
    ];
    
    Json(fake_users)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let app = Router::new()
        .route("/", get(hello_world))        
        .route("/health", get(health_check))    
        .route("/users", get(get_users))        
        .route("/users", post(create_user))
        .layer(cors);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:3000");
    println!("CORS enabled - accepting requests from any origin");
    println!("Try these URLs:");
    println!("  GET  http://localhost:3000/");
    println!("  GET  http://localhost:3000/health");
    println!("  GET  http://localhost:3000/users");
    println!("  POST http://localhost:3000/users (send JSON data!)");
    println!("To test POST, use curl:");
    println!("curl -X POST http://localhost:3000/users \\");
    println!("  -H 'Content-Type: application/json' \\");
    println!("  -d '{{\"name\":\"John\",\"email\":\"john@example.com\",\"age\":25}}'");
    
    axum::serve(listener, app).await.unwrap();
}
