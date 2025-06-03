# 🚀 Axum Backend Learning Project

A basic Axum web server demonstrating fundamental HTTP concepts and RESTful API patterns. Built for learning backend development with Rust.

## 📚 What This Covers

### HTTP Fundamentals
- **GET**: Retrieve data from server
- **POST**: Create new resources
- **PUT**: Update entire resource
- **PATCH**: Update part of resource  
- **DELETE**: Remove resource

### HTTP Status Codes
- **200**: OK - Request successful
- **201**: Created - Resource created successfully
- **204**: No Content - Successful but nothing to return
- **400**: Bad Request - Client error
- **401**: Unauthorized - Authentication required
- **403**: Forbidden - Access denied
- **404**: Not Found - Resource doesn't exist
- **500**: Internal Server Error - Server error

### RESTful API Pattern
```
GET    /users      → List all users
POST   /users      → Create new user  
GET    /users/123  → Get user #123
PUT    /users/123  → Update entire user #123
PATCH  /users/123  → Update part of user #123
DELETE /users/123  → Delete user #123
```

## 🛠️ What This Server Does

### Routes Available:
- `GET /` - Hello world message
- `GET /health` - Health check endpoint
- `GET /users` - List all users (fake data)
- `POST /users` - Create a new user

### Features Implemented:
- JSON request/response handling
- Proper HTTP status codes
- Request data validation (basic)
- Structured error responses
- Console logging for debugging

## 🏃‍♂️ How to Run

### Prerequisites
- Rust installed ([rustup.rs](https://rustup.rs/))
- Basic understanding of Rust syntax

### Setup
1. Clone this repository
2. Navigate to project directory
3. Install dependencies:
   ```bash
   cargo build
   ```

### Dependencies (Cargo.toml)
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### Run the Server
```bash
cargo run
```

Server will start on `http://localhost:3000`

## 🧪 Testing the API

### Test GET Requests (Browser or curl)
```bash
# Hello world
curl http://localhost:3000/

# Health check
curl http://localhost:3000/health

# List users
curl http://localhost:3000/users
```

### Test POST Request (curl required)
```bash
curl -X POST http://localhost:3000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"John","email":"john@example.com","age":25}'
```

### Expected Response for POST:
```json
{
  "id": 42,
  "name": "John",
  "email": "john@example.com", 
  "age": 25,
  "message": "User created successfully!"
}
```

## 🔍 Code Structure Explained

### Key Concepts:

**Handler Functions**
- Async functions that process HTTP requests
- Return responses with proper status codes
- Can extract JSON data from request body

**Router**
- Maps URL paths to handler functions
- Supports different HTTP methods on same path
- Example: `GET /users` vs `POST /users`

**JSON Serialization**
- `#[derive(Serialize)]` - Convert Rust struct to JSON response
- `#[derive(Deserialize)]` - Parse JSON request to Rust struct
- Automatic validation of required fields

**Status Codes**
- `StatusCode::OK` (200) for successful GET
- `StatusCode::CREATED` (201) for successful POST
- Returned as tuple: `(StatusCode, Json<Response>)`

## 🎯 Next Learning Steps

### What to Add Next:
1. **Path Parameters**: `GET /users/{id}` to get specific user
2. **DELETE Route**: Remove users from the system
3. **PUT/PATCH Routes**: Update existing users
4. **Database Integration**: Replace fake data with real database
5. **Error Handling**: Proper error responses for validation failures
6. **Authentication**: Protect routes with JWT tokens
7. **Middleware**: Logging, CORS, rate limiting
8. **Testing**: Unit and integration tests

### Advanced Topics to Explore:
- Database connections (SQLx, Diesel)
- Environment configuration
- Docker containerization
- API documentation (OpenAPI/Swagger)
- Production deployment
- Monitoring and metrics

## 📖 Learning Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [HTTP Status Codes Reference](https://httpstatuses.com/)
- [REST API Design Best Practices](https://restfulapi.net/)

## 💡 Learning Notes

- Always use proper HTTP methods for their intended purpose
- Status codes communicate what happened to the client
- JSON serialization/deserialization is handled automatically by Serde
- `async/await` is essential for handling concurrent requests
- Start simple, add complexity gradually
- Test every endpoint you create

---

**Next**: Continue building features and exploring advanced backend concepts.