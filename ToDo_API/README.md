# Todo API Documentation

## Overview

This is a simple RESTful API built with Rust and the Warp framework for managing a list of todos. It supports CRUD operations (Create, Read, Update, Delete) for todo items, stored in-memory. The API is designed to be lightweight, fast, and extensible.

## Features

- Create a new todo with a title and completion status.
- Retrieve all todos.
- Update an existing todo by ID.
- Delete a todo by ID.
- In-memory storage (data is not persisted across server restarts).

## Prerequisites

- **Rust**: Install Rust and Cargo using `rustup`:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- A tool for sending HTTP requests (e.g., `curl`, Postman).

## Setup

1. **Clone the Project** (if applicable, or create a new one):

   ```bash
   cargo new todo-api
   cd todo-api
   ```

2. **Update** `Cargo.toml`: Ensure the following dependencies are included:

   ```toml
   [package]
   name = "todo-api"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   warp = "0.3"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   tokio = { version = "1.0", features = ["full"] }
   pretty_env_logger = "0.5"
   uuid = { version = "1.0", features = ["v4"] }
   log = "0.4"
   ```

3. **Add the Code**: Place the provided `main.rs` code in `src/main.rs`. (Refer to the project source code for the full implementation.)

4. **Run the Server**:

   ```bash
   cargo run
   ```

   The server will start at `http://127.0.0.1:3030`.

## API Endpoints

### 1. GET /todos

Retrieve all todos.

- **Method**: GET
- **URL**: `/todos`
- **Response**:

  - **Status**: `200 OK`
  - **Body**: JSON array of todo objects.
  - **Example**:

    ```json
    [
      {
        "id": "0981ebc0-d516-44cc-ae83-858d677bc924",
        "title": "Learn Rust",
        "completed": false
      },
      {
        "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
        "title": "Build API",
        "completed": true
      }
    ]
    ```

### 2. POST /todos

Create a new todo.

- **Method**: POST
- **URL**: `/todos`
- **Request Body**:

  ```json
  {
    "title": "string",
    "completed": boolean
  }
  ```

- **Response**:

  - **Status**: `201 Created`
  - **Body**: JSON object of the created todo with a generated `id`.
  - **Example**:

    ```bash
    curl -X POST http://127.0.0.1:3030/todos -H "Content-Type: application/json" -d '{"title":"Learn Rust","completed":false}'
    ```

    ```json
    {
      "id": "0981ebc0-d516-44cc-ae83-858d677bc924",
      "title": "Learn Rust",
      "completed": false
    }
    ```

### 3. PUT /todos/:id

Update an existing todo by ID.

- **Method**: PUT
- **URL**: `/todos/{id}`
- **Request Body**:

  ```json
  {
    "title": "string",
    "completed": boolean
  }
  ```

- **Response**:

  - **Status**: `200 OK` (if found) or `404 Not Found` (if ID does not exist)
  - **Body**: JSON object of the updated todo.
  - **Example**:

    ```bash
    curl -X PUT http://127.0.0.1:3030/todos/0981ebc0-d516-44cc-ae83-858d677bc924 -H "Content-Type: application/json" -d '{"title":"Learn Rust Updated","completed":true}'
    ```

    ```json
    {
      "id": "0981ebc0-d516-44cc-ae83-858d677bc924",
      "title": "Learn Rust Updated",
      "completed": true
    }
    ```

### 4. DELETE /todos/:id

Delete a todo by ID.

- **Method**: DELETE
- **URL**: `/todos/{id}`
- **Response**:

  - **Status**: `204 No Content` (if deleted) or `404 Not Found` (if ID does not exist)
  - **Body**: JSON string `"Todo deleted"`.
  - **Example**:

    ```bash
    curl -X DELETE http://127.0.0.1:3030/todos/0981ebc0-d516-44cc-ae83-858d677bc924
    ```

    ```json
    "Todo deleted"
    ```

## Error Handling

- **400 Bad Request**: Returned for invalid JSON or missing required fields in the request body.
- **404 Not Found**: Returned when the specified todo ID does not exist.
- **Example**:

  ```json
  { "message": "Not found" }
  ```

## Notes

- **Storage**: Todos are stored in-memory and will be lost when the server restarts. For persistent storage, consider integrating a database (e.g., SQLite or PostgreSQL) using crates like `sqlx` or `diesel`.
- **Logging**: The API uses `pretty_env_logger` for logging server events. Logs are output to the console when running `cargo run`.
- **Extensibility**: The Warp framework allows easy addition of features like authentication, CORS, or additional endpoints.

## Troubleshooting

- **Server not starting**: Ensure all dependencies in `Cargo.toml` are correct and run `cargo build` to check for compilation errors.
- **Bad request errors**: Verify the JSON payload matches the expected format (`title` as a string, `completed` as a boolean).
- **404 errors**: Ensure the todo ID exists by checking `GET /todos` before updating or deleting.

## Future Improvements

- Add database integration for persistent storage.
- Implement authentication and authorization (e.g., JWT).
- Add query parameters for filtering todos (e.g., by completion status).

Enhance error messages with more details.
