// Cargo.toml dependencies you'll need:
// [dependencies]
// axum = "0.7"
// tokio = { version = "1.0", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }

use axum::{
    extract::Json as ExtractJson,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

// What we expect from the client when creating a user
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u8,
}

// What we send back after creating a user
#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
    email: String,
    age: u8,
    message: String,
}

// This is what we'll return as JSON
#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

// Our first route handler - just like the drive-through worker
async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, future backend god!".to_string(),
        status: "success".to_string(),
    })
}

// Another handler that shows different HTTP responses
async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Server is alive and kicking!")
}

// POST handler - This is where the magic happens!
async fn create_user(ExtractJson(payload): ExtractJson<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
    // In real life, you'd save this to a database
    // For now, we'll just pretend and generate a fake ID
    let fake_user_id = 42; // In reality: database.insert(user).id
    
    // Validate the data (basic example)
    if payload.name.is_empty() {
        // You could return a 400 Bad Request here
        // We'll keep it simple for now
    }
    
    println!("🎉 Creating user: {} ({})", payload.name, payload.email);
    
    let response = CreateUserResponse {
        id: fake_user_id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
        message: "User created successfully!".to_string(),
    };
    
    // Return 201 Created with the new user data
    (StatusCode::CREATED, Json(response))
}

// GET handler for listing users (bonus!)
async fn get_users() -> Json<Vec<serde_json::Value>> {
    // Fake user data - in real life this comes from database
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
    // Build our application with routes
    let app = Router::new()
        .route("/", get(hello_world))           // GET / -> hello_world()
        .route("/health", get(health_check))    // GET /health -> health_check()
        .route("/users", get(get_users))        // GET /users -> get_users()
        .route("/users", post(create_user));    // POST /users -> create_user()

    // Start the server
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