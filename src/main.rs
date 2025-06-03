use axum::{
    extract::Json as ExtractJson,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

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
struct HelloResponse {
    message: String,
    status: String,
}

async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, nigga!".to_string(),
        status: "success".to_string(),
    })
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Server is alive and kicking!")
}

async fn create_user(ExtractJson(payload): ExtractJson<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
    let fake_user_id = 42;
    
    if payload.name.is_empty() {
        // 400 Bad Request here
    }
    
    println!("🎉 Creating user: {} ({})", payload.name, payload.email);
    
    let response = CreateUserResponse {
        id: fake_user_id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
        message: "User created successfully!".to_string(),
    };

    (StatusCode::CREATED, Json(response))
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
    let app = Router::new()
        .route("/", get(hello_world))        
        .route("/health", get(health_check))    
        .route("/users", get(get_users))        
        .route("/users", post(create_user));    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://localhost:3000");
    println!("Try these URLs:");
    println!("  GET  http://localhost:3000/");
    println!("  GET  http://localhost:3000/health");
    println!("  GET  http://localhost:3000/users");
    println!("  POST http://localhost:3000/users (send JSON data!)");
    println!("\nTo test POST, use curl:");
    println!("curl -X POST http://localhost:3000/users \\");
    println!("  -H 'Content-Type: application/json' \\");
    println!("  -d '{{\"name\":\"John\",\"email\":\"john@example.com\",\"age\":25}}'");
    
    axum::serve(listener, app).await.unwrap();
}