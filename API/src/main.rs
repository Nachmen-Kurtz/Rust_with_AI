use log::info;
use pretty_env_logger;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply, http::StatusCode};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
  id: String,
  title: String,
  completed: bool,
}

#[derive(Debug, Deserialize)]
struct NewTodo {
  title: String,
  completed: bool,
}

type Todos = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  let todos: Todos = Arc::new(Mutex::new(Vec::new()));

  // GET /todos
  let get_todos = warp::path("todos")
    .and(warp::get())
    .and(with_todos(todos.clone()))
    .and_then(get_todos_handler);

  // POST /todos
  let create_todo = warp::path("todos")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_todos(todos.clone()))
    .and_then(create_todo_handler);

  // PUT /todos/:id
  let update_todo = warp::path!("todos" / String)
    .and(warp::put())
    .and(warp::body::json())
    .and(with_todos(todos.clone()))
    .and_then(update_todo_handler);

  // DELETE /todos/:id
  let delete_todo = warp::path!("todos" / String)
    .and(warp::delete())
    .and(with_todos(todos.clone()))
    .and_then(delete_todo_handler);

  let routes = get_todos
    .or(create_todo)
    .or(update_todo)
    .or(delete_todo)
    .recover(handle_rejection);

  info!("Starting server at 127.0.0.1:3030");
  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_todos(
  todos: Todos,
) -> impl Filter<Extract = (Todos,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || todos.clone())
}

async fn get_todos_handler(todos: Todos) -> Result<impl Reply, Rejection> {
  let todos = todos.lock().unwrap();
  Ok(warp::reply::json(&*todos))
}

async fn create_todo_handler(new_todo: NewTodo, todos: Todos) -> Result<impl Reply, Rejection> {
  let mut todos = todos.lock().unwrap();
  let todo = Todo {
    id: Uuid::new_v4().to_string(),
    title: new_todo.title,
    completed: new_todo.completed,
  };
  todos.push(todo.clone());
  Ok(warp::reply::with_status(
    warp::reply::json(&todo),
    StatusCode::CREATED,
  ))
}

async fn update_todo_handler(
  id: String,
  updated_todo: NewTodo,
  todos: Todos,
) -> Result<impl Reply, Rejection> {
  let mut todos = todos.lock().unwrap();
  if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
    todo.title = updated_todo.title;
    todo.completed = updated_todo.completed;
    Ok(warp::reply::json(&todo))
  } else {
    Err(warp::reject::not_found())
  }
}

async fn delete_todo_handler(id: String, todos: Todos) -> Result<impl Reply, Rejection> {
  let mut todos = todos.lock().unwrap();
  if let Some(pos) = todos.iter().position(|t| t.id == id) {
    todos.remove(pos);
    Ok(warp::reply::with_status(
      warp::reply::json(&"Todo deleted"),
      StatusCode::NO_CONTENT,
    ))
  } else {
    Err(warp::reject::not_found())
  }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
  if err.is_not_found() {
    Ok(warp::reply::with_status(
      warp::reply::json(&"Not found"),
      StatusCode::NOT_FOUND,
    ))
  } else {
    Ok(warp::reply::with_status(
      warp::reply::json(&"Bad request"),
      StatusCode::BAD_REQUEST,
    ))
  }
}
